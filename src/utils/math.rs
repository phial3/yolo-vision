use opencv::core::Rect;

/// 计算两个矩形的交并比
pub fn calculate_iou(a: &Rect, b: &Rect) -> f32 {
    // 计算两个矩形的交集
    let x_left = a.x.max(b.x);
    let y_top = a.y.max(b.y);
    let x_right = (a.x + a.width).min(b.x + b.width);
    let y_bottom = (a.y + a.height).min(b.y + b.height);

    // 检查是否有交集
    if x_right < x_left || y_bottom < y_top {
        return 0.0; // 没有交集，IoU 为 0
    }

    // 交集面积
    let intersection_area = (x_right - x_left) * (y_bottom - y_top);

    // 各自的面积
    let a_area = a.width * a.height;
    let b_area = b.width * b.height;

    // 检查是否存在并集
    if a_area == 0 || b_area == 0 {
        return 0.0; // 其中一个矩形没有面积，IoU 为 0
    }

    // 并集面积 = a 面积 + b 面积 - 交集面积
    let union_area = a_area + b_area - intersection_area;

    // 计算 IoU
    intersection_area as f32 / union_area as f32
}
