import cv2
import numpy as np
import onnxruntime

# 加载 ONNX 模型
onnx_session = onnxruntime.InferenceSession("yolov5.onnx")

# 设置置信度阈值和 NMS 阈值
confThreshold = 0.25  # 50% 置信度阈值
nmsThreshold = 0.45   # 45% NMS 阈值

# 预处理输入图像
def preprocess(image, input_size):
    img = cv2.resize(image, (input_size, input_size))
    img = img.astype(np.float32)
    img /= 255.0  # 归一化
    img = np.transpose(img, (2, 0, 1))  # HWC to CHW
    img = np.expand_dims(img, axis=0)
    return img

# 后处理，执行 NMS 和过滤检测框
def postprocess(predictions, confThreshold, nmsThreshold):
    # 解析 ONNX 输出，应用置信度阈值和 NMS
    boxes, scores, class_ids = [], [], []
    # 执行 NMS 等操作
    return boxes, scores, class_ids

# 推理阶段
image = cv2.imread('input_image.jpg')
input_size = 640
input_image = preprocess(image, input_size)

# 推理
input_name = onnx_session.get_inputs()[0].name
output = onnx_session.run(None, {input_name: input_image})

# 后处理
boxes, scores, class_ids = postprocess(output, confThreshold, nmsThreshold)

