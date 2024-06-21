use video_rs::{self, encode::Settings, Encoder, Location, Time};

fn main() {
    let width = 800_i32;
    let height = 600_i32;
    let color_bg = [240, 240, 240];
    let color_cube = [197, 0, 0];
    let destination = Location::File(std::path::PathBuf::from("BigBuckBunny.mp4"));

    let encoder_settings = Settings::preset_h264_yuv420p(width as usize, height as usize, false);
    let mut encoder =
        Encoder::new(&destination, encoder_settings).expect("failed to create encoder");

    let duration: Time = std::time::Duration::from_nanos(1_000_000_000 / 60).into();
    let mut position = Time::zero();

    let center_x = width / 2;
    let center_y = height / 2;
    for size in 4..520 {
        // Using some Pythagoras magic to draw a circle that grows bigger and bigger!
        let frame =
            ndarray::Array3::from_shape_fn((height as usize, width as usize, 3), |(y, x, c)| {
                let dx = (x as i32 - center_x).abs();
                let dy = (y as i32 - center_y).abs();
                let d = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();
                if d < size.into() {
                    color_cube[c]
                } else {
                    color_bg[c]
                }
            });
        let rgb = frame.as_slice().unwrap();
        println!("{:?}", rgb.to_vec().len() / 3);

        encoder
            .encode(&frame, position)
            .expect("failed to encode frame");
        position = position.aligned_with(duration).add();
    }

    encoder.finish().expect("failed to finish encoding");
}
