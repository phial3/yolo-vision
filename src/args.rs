use anyhow::Result;
use once_cell::sync::Lazy;
use usls::Options;

static ARGS: Lazy<Args> = Lazy::new(argh::from_env);

#[derive(argh::FromArgs, Debug)]
/// Example
pub(crate) struct Args {
    /// model file
    #[argh(option)]
    model: Option<String>,

    /// source
    #[argh(option, default = "String::from(\"./assets/bus.jpg\")")]
    source: String,

    /// output
    #[argh(option, default = "String::from(\"./output.mp4\")")]
    output: String,

    /// dtype
    #[argh(option, default = "String::from(\"auto\")")]
    dtype: String,

    /// task
    #[argh(option, default = "String::from(\"det\")")]
    task: String,

    /// version
    #[argh(option, default = "8.0")]
    ver: f32,

    /// device
    #[argh(option, default = "String::from(\"cpu:0\")")]
    device: String,

    /// scale
    #[argh(option, default = "String::from(\"n\")")]
    scale: String,

    /// trt_fp16
    #[argh(option, default = "true")]
    trt_fp16: bool,

    /// find_contours
    #[argh(option, default = "true")]
    find_contours: bool,

    /// batch_size
    #[argh(option, default = "1")]
    batch_size: usize,

    /// min_batch_size
    #[argh(option, default = "1")]
    min_batch_size: usize,

    /// max_batch_size
    #[argh(option, default = "4")]
    max_batch_size: usize,

    /// min_image_width
    #[argh(option, default = "224")]
    min_image_width: isize,

    /// image_width
    #[argh(option, default = "640")]
    image_width: isize,

    /// max_image_width
    #[argh(option, default = "1280")]
    max_image_width: isize,

    /// min_image_height
    #[argh(option, default = "224")]
    min_image_height: isize,

    /// image_height
    #[argh(option, default = "640")]
    image_height: isize,

    /// max_image_height
    #[argh(option, default = "1280")]
    max_image_height: isize,

    /// num_classes
    #[argh(option)]
    num_classes: Option<usize>,

    /// num_keypoints
    #[argh(option)]
    num_keypoints: Option<usize>,

    /// use_coco_80_classes
    #[argh(switch)]
    use_coco_80_classes: bool,

    /// use_imagenet_1k_classes
    #[argh(switch)]
    use_imagenet_1k_classes: bool,

    /// confs
    #[argh(option)]
    confs: Vec<f32>,

    /// keypoint_confs
    #[argh(option)]
    keypoint_confs: Vec<f32>,

    /// exclude_classes
    #[argh(option)]
    exclude_classes: Vec<usize>,

    /// retain_classes
    #[argh(option)]
    retain_classes: Vec<usize>,

    /// class_names
    #[argh(option)]
    class_names: Vec<String>,

    /// keypoint_names
    #[argh(option)]
    keypoint_names: Vec<String>,
}

pub(crate) fn instance() -> &'static Args {
    &ARGS
}

pub fn input_source() -> String {
    instance().source.clone()
}

pub fn output() -> String {
    instance().output.clone()
}

pub fn build_options() -> Result<Options> {
    let args = instance();

    let mut options = Options::yolo()
        .with_model_file(args.model.as_ref().unwrap_or(&String::new()))
        .with_model_task(args.task.as_str().try_into()?)
        .with_model_version(args.ver.into())
        .with_model_scale(args.scale.as_str().try_into()?)
        .with_model_dtype(args.dtype.as_str().try_into()?)
        .with_model_device(args.device.as_str().try_into()?)
        .with_trt_fp16(args.trt_fp16)
        .with_model_ixx(
            0,
            0,
            (args.min_batch_size, args.batch_size, args.max_batch_size).into(),
        )
        .with_model_ixx(
            0,
            2,
            (
                args.min_image_height,
                args.image_height,
                args.max_image_height,
            )
                .into(),
        )
        .with_model_ixx(
            0,
            3,
            (args.min_image_width, args.image_width, args.max_image_width).into(),
        )
        .with_class_confs(if args.confs.is_empty() {
            &[0.2, 0.15]
        } else {
            &args.confs
        })
        .with_keypoint_confs(if args.keypoint_confs.is_empty() {
            &[0.5]
        } else {
            &args.keypoint_confs
        })
        .with_find_contours(args.find_contours)
        .retain_classes(&args.retain_classes)
        .exclude_classes(&args.exclude_classes);

    if args.use_coco_80_classes {
        options = options.with_class_names(&usls::COCO_CLASS_NAMES_80);
    }

    if args.use_imagenet_1k_classes {
        options = options.with_class_names(&usls::IMAGENET_NAMES_1K);
    }

    if let Some(nc) = args.num_classes {
        options = options.with_nc(nc);
    }

    if let Some(nk) = args.num_keypoints {
        options = options.with_nk(nk);
    }

    if !args.class_names.is_empty() {
        options = options.with_class_names(
            &args
                .class_names
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>(),
        );
    }

    if !args.keypoint_names.is_empty() {
        options = options.with_keypoint_names(
            &args
                .keypoint_names
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>(),
        );
    }

    Ok(options)
}
