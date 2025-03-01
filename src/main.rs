use anyhow::Result;
use cv_convert::TryFromCv;
use rsmedia::{time::Time, EncoderBuilder};
use rsmpeg::avutil::AVFrame;
use usls::{models::YOLO, Annotator, DataLoader};
use yolo_vision::args;

/// run: RUST_LOG=debug cargo run -- --source 'rtmp://172.24.82.44/live/livestream1' \
/// --output 'rtmp://172.24.82.44/live/livestream_dev' \
/// --model /Users/admin/Workspace/rust/rpi/models/v8/yolov8m.onnx
fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .init();

    let options = args::build_options()?;

    // build model
    let mut model = YOLO::try_from(options.commit()?)?;

    // build dataloader
    let dl = DataLoader::new(&args::input_source())?
        .with_batch(model.batch() as _)
        .build()?;

    // build annotator
    let annotator = Annotator::default()
        .with_skeletons(&usls::COCO_SKELETONS_16)
        .without_masks(true)
        .with_bboxes_thickness(3)
        .with_saveout(model.spec());

    let mut position = Time::zero();
    let duration: Time = Time::from_nth_of_a_second(24);

    let mut encoder = EncoderBuilder::new(std::path::Path::new(&args::output()), 1280, 720)
        .with_format("flv")
        .build()?;

    tracing::info!("model run and annotate start...");

    // run & annotate
    for (xs, _paths) in dl {
        let ys = model.forward(&xs)?;
        // extract bboxes
        // for y in ys.iter() {
        //     if let Some(bboxes) = y.bboxes() {
        //         println!("[Bboxes]: Found {} objects", bboxes.len());
        //         for (i, bbox) in bboxes.iter().enumerate() {
        //             println!("{}: {:?}", i, bbox)
        //         }
        //     }
        // }

        // plot
        let frames = annotator.plot(&xs, &ys, false)?;

        // encode
        for (i, img) in frames.iter().enumerate() {
            // save image
            img.save(format!("{}_{}.png", "/tmp/rsmedia_output", i))?;

            // image -> AVFrame 默认转换格式为 RGB24，在内部推流会自动转换为 YUV420P
            let mut frame = AVFrame::try_from_cv(&img.to_rgb8())?;
            // 设置帧的PTS
            frame.set_pts(position.into_value().unwrap());
            // 推流
            encoder.encode_raw(&frame)?;

            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(duration).add()
        }
    }

    model.summary();

    encoder.finish().expect("failed to finish encoder");

    Ok(())
}
