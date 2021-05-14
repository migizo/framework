use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
  let sdl = sdl2::init().expect("error sdl2 init");
  let video_subsystem = sdl.video().expect("error sdl2 video");
  let window_width = 1024;
  let window_height = 768;
  let window = video_subsystem
    .window("toolkit_test", window_width, window_height)
    .opengl()
    .position_centered()
    .resizable()
    .build()
    .expect("error window init");

  let mut event_pump = sdl.event_pump().unwrap();
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => (),
      }
    }
    window.gl_swap_window();
  }
  ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
}
