use image::{ImageBuffer, Rgb};

fn main() {
    // 创建一个 2x4 的 RGB 图像
    let width = 2;
    let height = 4;
    let buffer: Vec<u8> = vec![
        0, 0, 0, // 黑色
        0, 0, 255, // 蓝色
        0, 255, 0, // 绿色
        0, 255, 255, // 青色
        255, 0, 0, // 红色
        255, 0, 255, // 品红色
        255, 255, 0, // 黄色
        255, 255, 255, // 白色
    ];

    // 使用 image crate 将 buffer 转换为 ImageBuffer
    let image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, buffer).unwrap();

    // 操作图像或保存图像
    image_buffer
        .save_with_format("/tmp/rgb_image.png", image::ImageFormat::Png)
        .expect("save image failed");
}
