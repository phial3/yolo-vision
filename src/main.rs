use anyhow::Result;
use crossbeam::queue::SegQueue;
use image::DynamicImage;
use parking_lot::Mutex;
use rayon::prelude::*;
use tokio::sync::mpsc;

use std::sync::Arc;
use std::time::{Duration, Instant};

use cv_convert::TryFromCv;
use rsmedia::hwaccel::HWDeviceType;
use rsmedia::{EncoderBuilder, Options, RawFrame};
use usls::{models::YOLO, Annotator, DataLoader, Device};
use yolo_vision::args;

/// run: RUST_LOG=debug cargo run -- --source 'rtmp://172.24.82.44/live/livestream1' \
/// --output 'rtmp://172.24.82.44/live/livestream_dev' \
/// --model /Users/admin/Workspace/rust/rpi/models/v8/yolov8m.onnx
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .init();

    let options = args::build_options()?;

    // 将model包装在Arc<Mutex>中以支持可变访问
    let model = Arc::new(Mutex::new(YOLO::try_from(options.commit()?)?));

    // build dataloader
    let dl = DataLoader::new(&args::input_source())?
        .with_batch(model.lock().batch() as _)
        .with_device(Device::Cuda(0))
        .build()?;

    // build annotator
    let annotator = Arc::new(
        Annotator::default()
            .with_skeletons(&usls::COCO_SKELETONS_16)
            .without_masks(true)
            .with_bboxes_thickness(3)
            .with_saveout(model.lock().spec()),
    );

    // let position = Arc::new(Mutex::new(Time::zero()));
    // let duration = Time::from_nth_of_a_second(24);

    // build encoder
    let encoder = Arc::new(Mutex::new(
        EncoderBuilder::new(std::path::Path::new(&args::output()), 1280, 720)
            .with_format("flv")
            .with_codec_name("h264_nvenc".to_string())
            .with_hardware_device(HWDeviceType::CUDA)
            .with_options(&Options::preset_h264_nvenc())
            .with_thread_count(16)
            .build()?,
    ));

    tracing::info!("model run and annotate start...");

    // 创建性能统计计数器
    let inference_times = Arc::new(SegQueue::new());
    let annotation_times = Arc::new(SegQueue::new());
    let encoding_times = Arc::new(SegQueue::new());
    let encoding_times_clone = Arc::clone(&encoding_times);

    // 创建用于帧处理的channel
    let (frame_tx, mut frame_rx) = mpsc::channel::<DynamicImage>(32);

    // 启动编码任务
    let encode_handle = {
        let encoder = Arc::clone(&encoder);
        // let position = Arc::clone(&position);
        // let duration = duration;

        tokio::spawn(async move {
            while let Some(frame) = frame_rx.recv().await {
                let encode_start = Instant::now();

                let raw_frame = match RawFrame::try_from_cv(&frame.to_rgb8()) {
                    Ok(rf) => rf,
                    Err(e) => {
                        tracing::error!("Failed to convert frame to AVFrame: {:?}", e);
                        continue;
                    }
                };

                tokio::task::block_in_place(|| {
                    if let Err(e) = encoder.lock().encode_raw(&raw_frame) {
                        tracing::error!("Failed to encode frame: {:?}", e);
                    }
                });

                // 记录编码时间
                encoding_times_clone.push(encode_start.elapsed());
            }
        })
    };

    // 主处理循环
    let mut batch_count = 0;
    for (xs, _paths) in dl {
        let inference_start = Instant::now();
        let ys = match model.lock().forward(&xs) {
            Ok(y) => {
                inference_times.push(inference_start.elapsed());
                y
            }
            Err(e) => {
                tracing::error!("Model inference failed: {:?}", e);
                continue;
            }
        };

        let annotation_start = Instant::now();
        let frames = match annotator.plot(&xs, &ys, false) {
            Ok(f) => {
                annotation_times.push(annotation_start.elapsed());
                f
            }
            Err(e) => {
                tracing::error!("Frame annotation failed: {:?}", e);
                continue;
            }
        };

        let batch_sender_start = Instant::now();
        let processed_frames: Vec<_> = frames.into_par_iter().collect();
        // 在异步上下文中发送帧
        for frame in processed_frames {
            if let Err(e) = frame_tx.send(frame).await {
                tracing::error!("Failed to send frame to encoder: {:?}", e);
                continue;
            }
        }

        batch_count += 1;
        if batch_count % 10 == 0 {
            // 从 SegQueue 中获取所有元素
            let inference_stats = collect_and_calculate_stats(&inference_times);
            let annotation_stats = collect_and_calculate_stats(&annotation_times);
            let encoding_stats = collect_and_calculate_stats(&encoding_times.clone());

            tracing::info!(
                "Performance stats after 1 batches:\n\
                 Inference: avg={:?}, min={:?}, max={:?}\n\
                 Annotation: avg={:?}, min={:?}, max={:?}\n\
                 Encoding: avg={:?}, min={:?}, max={:?}\n\
                 Batch sender time: {:?}",
                inference_stats.0,
                inference_stats.1,
                inference_stats.2,
                annotation_stats.0,
                annotation_stats.1,
                annotation_stats.2,
                encoding_stats.0,
                encoding_stats.1,
                encoding_stats.2,
                batch_sender_start.elapsed(),
            );
        }
    }

    // 关闭frame channel
    drop(frame_tx);

    // 等待编码任务完成
    if let Err(e) = encode_handle.await {
        tracing::error!("Encoder task failed: {:?}", e);
    }

    // 打印模型统计信息 - 使用 lock() 获取可变访问
    model.lock().summary();

    // 完成编码
    encoder.lock().finish()?;

    Ok(())
}

/// 从 SegQueue 中收集所有元素并计算统计信息
fn collect_and_calculate_stats(queue: &SegQueue<Duration>) -> (Duration, Duration, Duration) {
    // 逐个弹出元素，直到队列为空
    let mut times = Vec::new();
    while let Some(duration) = queue.pop() {
        times.push(duration);
    }
    calculate_stats(&times)
}

/// 计算统计信息 (avg, min, max)
fn calculate_stats(times: &[Duration]) -> (Duration, Duration, Duration) {
    if times.is_empty() {
        return (
            Duration::default(),
            Duration::default(),
            Duration::default(),
        );
    }

    let sum: Duration = times.iter().sum();
    let avg = sum / times.len() as u32;
    let min = *times.iter().min().unwrap();
    let max = *times.iter().max().unwrap();

    (avg, min, max)
}
