use opencv::{
    core::{Mat, Point, Rect, Scalar, Size, Vector},
    imgcodecs, imgproc,
    prelude::*,
    stitching::{Stitcher, Stitcher_Mode, Stitcher_Status},
    Result,
};

fn main() -> Result<()> {
    // 加载图像
    let images = vec![
        imgcodecs::imread("assets/image-1.jpg", imgcodecs::IMREAD_COLOR)?,
        imgcodecs::imread("assets/image-2.jpg", imgcodecs::IMREAD_COLOR)?,
        imgcodecs::imread("assets/image-3.jpg", imgcodecs::IMREAD_COLOR)?,
    ];
    // 使用 Vector<Mat> 作为输入
    let images_vector: Vector<Mat> = Vector::from(images);

    // 创建一个拼接器
    let mut stitcher = Stitcher::create(Stitcher_Mode::PANORAMA)?;

    // 创建全景图像的 Mat
    let mut pano = Mat::default();

    // 进行拼接
    let status = stitcher.stitch(&images_vector, &mut pano)?;
    if status == Stitcher_Status::OK {
        // 裁剪黑边
        // let cropped_image = crop_black_borders(&pano)?;
        // 保存拼接结果
        let output_image = "/tmp/panorama.jpg";
        imgcodecs::imwrite(output_image, &pano, &Vector::new())?;
        println!("全景图像已保存为 {:?}", output_image);
    } else {
        eprintln!("拼接失败，错误代码: {:?}", status);
    }

    Ok(())
}

pub fn crop_black_borders(image: &Mat) -> Result<Mat> {
    // 创建一个边框
    let border_size = 2;
    let bordered_image = Mat::new_size_with_default(
        Size::new(
            image.cols() + 2 * border_size,
            image.rows() + 2 * border_size,
        ),
        opencv::core::CV_8UC3,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )?;

    // 计算 ROI 区域
    let roi_rect = Rect::new(border_size, border_size, image.cols(), image.rows());
    let mut roi = bordered_image.roi(roi_rect)?.try_clone()?;
    // 将原图像复制到 ROI 区域
    image.copy_to(&mut roi)?;

    // 转换为灰度图像
    let mut gray = Mat::default();
    imgproc::cvt_color_def(&bordered_image, &mut gray, imgproc::COLOR_BGR2GRAY)?;

    // 进行二值化处理
    // let mut thresh = Mat::default();
    // imgproc::threshold(&gray, &mut thresh, 0.0, 255.0, imgproc::THRESH_BINARY)?;

    // 应用高斯模糊
    let mut blurred = Mat::default();
    imgproc::gaussian_blur_def(&gray, &mut blurred, Size::new(5, 5), 0.0)?;

    // 使用边缘检测，替代二值化步骤
    let mut edges = Mat::default();
    imgproc::canny(&blurred, &mut edges, 100.0, 200.0, 3, false)?;

    // 查找轮廓
    let mut contours: Vector<Vector<Point>> = Vector::new();
    imgproc::find_contours(
        &edges,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::default(),
    )?;

    println!("找到 {:?} 个轮廓", contours);

    // 找到最大轮廓
    let largest_contour = contours
        .iter()
        .max_by(|a, b| {
            let area_a = imgproc::contour_area(a, false).unwrap_or(0.0);
            let area_b = imgproc::contour_area(b, false).unwrap_or(0.0);
            area_a
                .partial_cmp(&area_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or_else(|| opencv::Error::new(opencv::core::StsError, "No contours found"))?;

    // 获取最大轮廓的边界框
    let bounding_rect = imgproc::bounding_rect(&largest_contour)?;

    // 使用边界框裁剪图像
    let cropped_image = bordered_image.roi(bounding_rect)?;

    Ok(cropped_image.try_clone()?)
}
