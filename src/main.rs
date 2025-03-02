use anyhow::Result;
use cv_convert::TryFromCv;
use rsmedia::hwaccel::HWDeviceType;
use rsmedia::{time::Time, EncoderBuilder, Options, RawFrame};
use usls::{models::YOLO, Annotator, DataLoader, Device};
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
        .with_device(Device::Cuda(0))
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
        .with_codec_name("h264_nvenc".to_string())
        .with_hardware_device(HWDeviceType::CUDA)
        .with_options(&Options::preset_h264_nvenc())
        .with_thread_count(10)
        .build()?;

    tracing::info!("model run and annotate start...");

    // run & annotate
    for (xs, _paths) in dl {
        let ys = model.forward(&xs)?;
        // extract bboxes
        for y in ys.iter() {
            if let Some(bboxes) = y.bboxes() {
                println!("[Bboxes]: Found {} objects", bboxes.len());
                for (i, bbox) in bboxes.iter().enumerate() {
                    println!("{}: {:?}", i, bbox)
                }
            }
        }

        // plot
        let frames = annotator.plot(&xs, &ys, false)?;

        // encode
        for (i, img) in frames.iter().enumerate() {
            // save image if needed
            img.save(format!("/tmp/images/{}_{}.png", string_now("-"), i))?;

            // image -> AVFrame
            let raw_frame = RawFrame::try_from_cv(&img.to_rgb8())?;

            // realtime streaming encoding
            encoder.encode_raw(&raw_frame)?;

            // Update the current position and add the inter-frame duration to it.
            position = position.aligned_with(duration).add()
        }
    }

    model.summary();

    encoder.finish().expect("failed to finish encoder");

    Ok(())
}

pub(crate) fn string_now(delimiter: &str) -> String {
    let t_now = chrono::Local::now();
    let fmt = format!(
        "%Y{}%m{}%d{}%H{}%M{}%S{}%f",
        delimiter, delimiter, delimiter, delimiter, delimiter, delimiter
    );
    t_now.format(&fmt).to_string()
}
