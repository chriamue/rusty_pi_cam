use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;

fn main() {

    let mut camera = Camera::new(
        0, // index
        Some(CameraFormat::new_from(1920, 1080, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    //println!("{:?}", camera.supported_camera_controls().unwrap());
    // open stream
    let res = camera.open_stream().unwrap();
    println!("{:?}", res);
    loop {
        println!("tik");
        let frame = camera.frame().unwrap();
        println!("tok");
        println!("{}, {}", frame.width(), frame.height());
        frame.save("frame.jpg").unwrap();
    }
    let frame = camera.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();
}
