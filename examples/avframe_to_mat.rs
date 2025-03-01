// use anyhow::{Context, Result};
// use opencv::imgcodecs;
// use std::ffi::CString;
// use std::sync::atomic::Ordering;
// use yolo_vision::misc::{av_convert, avio};
//
// fn main() -> Result<()> {
//     // Path to the image file
//     let file_path = CString::new("assets/dog.jpg").unwrap();
//
//     // Open the input file
//     let (video_stream_index, mut input_format_context, mut decode_context) =
//         avio::open_input_file(file_path.as_c_str()).unwrap();
//
//     let img_index = std::sync::atomic::AtomicUsize::new(0);
//
//     // Read frames from the file
//     while let Some(packet) = input_format_context.read_packet()? {
//         if packet.stream_index == video_stream_index as i32 {
//             decode_context.send_packet(Some(&packet))?;
//
//             while let Ok(yuv_frame) = decode_context.receive_frame() {
//                 // Convert YUV to RGB24
//                 let rgb_frame = av_convert::avframe_yuv_to_rgb24(&yuv_frame)?;
//
//                 // Convert AVFrame to Mat
//                 let mat = av_convert::avframe_rgb24_to_mat(&rgb_frame)?;
//                 println!("Converted AVFrame to Mat successfully.");
//
//                 // Save
//                 let output_path = format!(
//                     "/tmp/write_mat_{}.jpg",
//                     img_index.fetch_add(1, Ordering::SeqCst)
//                 );
//                 imgcodecs::imwrite(&output_path, &mat, &opencv::core::Vector::new())
//                     .context(format!("Failed to write image to {}", output_path))?;
//             }
//         }
//     }
//
//     Ok(())
// }

fn main() {}
