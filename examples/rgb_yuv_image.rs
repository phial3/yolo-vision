use image::{Rgb, RgbImage};

fn main() {
    // 创建一个 4x4 的 RGB 图像
    let img_x = 4;
    let img_y = 4;

    // 定义图像的 RGB 数据
    let data = [
        [[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 0]], // 红, 绿, 蓝, 黄
        [[0, 255, 255], [255, 0, 255], [192, 192, 192], [128, 0, 0]], // 青, 品红, 灰, 深红
        [[0, 128, 0], [128, 128, 0], [0, 0, 128], [255, 165, 0]], // 绿, 橄榄, 深蓝, 橙色
        [[0, 0, 0], [255, 255, 255], [255, 69, 0], [173, 255, 47]], // 黑, 白, 橙红, 亮黄绿
    ];

    // 使用 `from_fn` 方法生成图像
    let img: RgbImage = RgbImage::from_fn(img_x, img_y, |x, y| {
        let pixel = data[y as usize][x as usize];
        Rgb([pixel[0], pixel[1], pixel[2]])
    });

    // 将图像保存为 PNG 文件
    img.save("/tmp/output.png").expect("Failed to save image");
}
