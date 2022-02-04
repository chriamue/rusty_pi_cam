use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;

fn main() {
    let mut camera = Camera::new(
        0,                                                                // index
        Some(CameraFormat::new_from(1920, 1080, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    let res = camera.open_stream().unwrap();
    println!("{:?}", res);
    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
        frame.save("frame.jpg").unwrap();
    }
}
