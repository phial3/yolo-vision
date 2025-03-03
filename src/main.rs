use anyhow::Result;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::sync::mpsc;

use cv_convert::TryFromCv;
use image::DynamicImage;
use rsmedia::hwaccel::HWDeviceType;
use rsmedia::{EncoderBuilder, Options, RawFrame};
use usls::{models::YOLO, Annotator, DataLoader, Device};
use yolo_vision::args;

/// run: RUST_LOG=debug cargo run -- --source 'rtmp://172.24.82.44/live/livestream1' \
/// --output 'rtmp://172.24.82.44/live/livestream_dev' \
/// --model /Users/admin/Workspace/rust/rpi/models/v8/yolov8m.onnx
#[tokio::main]
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
            .with_thread_count(10)
            .build()?,
    ));

    tracing::info!("model run and annotate start...");

    // 创建用于帧处理的channel
    let (frame_tx, mut frame_rx) = mpsc::channel::<DynamicImage>(32);

    // 启动编码任务
    let encode_handle = {
        let encoder = Arc::clone(&encoder);
        // let position = Arc::clone(&position);
        // let duration = duration;

        tokio::spawn(async move {
            while let Some(frame) = frame_rx.recv().await {
                let raw_frame = match RawFrame::try_from_cv(&frame.to_rgb8()) {
                    Ok(rf) => rf,
                    Err(e) => {
                        tracing::error!("Failed to convert frame to AVFrame: {:?}", e);
                        continue;
                    }
                };

                if let Err(e) = encoder.lock().encode_raw(&raw_frame) {
                    tracing::error!("Failed to encode frame: {:?}", e);
                    continue;
                }

                // 更新位置
                // let mut pos = position.lock();
                // *pos = pos.aligned_with(duration).add();
            }
        })
    };

    // 获取当前的运行时句柄
    // let rt = tokio::runtime::Handle::current();

    // 主处理循环
    for (xs, _paths) in dl {
        // 模型推理 - 使用 lock() 获取可变访问
        let ys = match model.lock().forward(&xs) {
            Ok(y) => y,
            Err(e) => {
                tracing::error!("Model inference failed: {:?}", e);
                continue;
            }
        };

        // 标注处理
        let frames = match annotator.plot(&xs, &ys, false) {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Frame annotation failed: {:?}", e);
                continue;
            }
        };

        // 收集处理后的帧
        let processed_frames: Vec<_> = frames.into_par_iter().collect();
        // 在异步上下文中发送帧
        for frame in processed_frames {
            if let Err(e) = frame_tx.send(frame).await {
                tracing::error!("Failed to send frame to encoder: {:?}", e);
                continue;
            }
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

#[allow(dead_code)]
pub(crate) fn string_now(delimiter: &str) -> String {
    let t_now = chrono::Local::now();
    let fmt = format!(
        "%Y{}%m{}%d{}%H{}%M{}%S{}%f",
        delimiter, delimiter, delimiter, delimiter, delimiter, delimiter
    );
    t_now.format(&fmt).to_string()
}
