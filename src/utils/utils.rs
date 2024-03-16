use instant::Instant;

pub fn calculate_fps(last_frame: Option<std::time::Instant>) -> f64 {
    let elapsed_seconds = last_frame.unwrap().elapsed().as_secs_f64();
    let fps = 1. / elapsed_seconds;
    //last_frame = Some(Instant::now());
    //println!("FPS: {fps}");

    fps
}