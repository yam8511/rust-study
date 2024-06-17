use show_image::{
    create_window,
    event::{self},
    WindowOptions,
};

fn main() {
    show_image::run_context(|| {
        let img = image::open("bus.jpg").unwrap();
        let win = create_window("demo", WindowOptions::default()).unwrap();
        win.set_image("bus", img.clone()).unwrap();

        let e = win.event_channel().unwrap();
        println!("event = {e:?}");
        let mut gray_switch = false;
        loop {
            let e = e.recv().unwrap();
            match e {
                show_image::event::WindowEvent::KeyboardInput(e) => {
                    match (e.input.state.is_pressed(), e.input.key_code) {
                        (true, Some(event::VirtualKeyCode::Q | event::VirtualKeyCode::Escape)) => {
                            break;
                        }
                        (true, Some(event::VirtualKeyCode::G)) => {
                            gray_switch = !gray_switch;
                            if gray_switch {
                                println!("to gray");
                                let img = img.clone().grayscale();
                                win.set_image("grayscale", img).unwrap();
                            } else {
                                println!("to normal");
                                win.set_image("normal", img.clone()).unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    });
}
