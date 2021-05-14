use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

static mut mouse: (i32, i32) = (0, 0);
static mut freq: f32 = 0.0;
static mut freqVol: f32 = 0.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 800;
fn main_sound() -> Result<(), anyhow::Error> {
  let host = cpal::default_host();
  let device = host
    .default_output_device()
    .expect("failed to find a default output device");

  loop {
    let config = device.default_output_config()?;

    match config.sample_format() {
      cpal::SampleFormat::F32 => run::<f32>(&device, &config.into())?,
      cpal::SampleFormat::I16 => run::<i16>(&device, &config.into())?,
      cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())?,
    }
  }

  Ok(())
}

fn calc_sin_wave(hz: f32, progress: f32) -> f32 {
  let pi = 3.141592;
  (hz * pi * 2.0 * progress).sin()
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
  T: cpal::Sample,
{
  let sample_rate = config.sample_rate.0 as f32; // ex.44.1k
  let channels = config.channels as usize; // ex.2ch

  // Produce a sinusoid of maximum amplitude.
  let mut sample_clock = 0f32;

  let mut next_value = move || {
    sample_clock = (sample_clock + 1.0) % sample_rate; // 0~44100

    let progress = sample_clock / sample_rate;
    let vol;
    let tonic;
    unsafe {
      tonic = calc_sin_wave(freq, progress);
      vol = calc_sin_wave(freqVol, progress);
    }
    tonic * vol
  };

  let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

  let stream = device.build_output_stream(
    config,
    move |data: &mut [T], _: &cpal::OutputCallbackInfo| write_data(data, channels, &mut next_value),
    err_fn,
  )?;
  stream.play()?;

  std::thread::sleep(std::time::Duration::from_millis(1000)); // 1sec

  Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
  T: cpal::Sample,
{
  for frame in output.chunks_mut(channels) {
    let value: T = cpal::Sample::from::<f32>(&next_sample());
    for sample in frame.iter_mut() {
      *sample = value;
    }
  }
}

//--------------------------------------------------------------------------------
use framework::mesh;
use framework::shader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use c_str_macro::c_str;
use cgmath::prelude::SquareMatrix;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;
fn setup_gl(sdl: &sdl2::Sdl, video_subsystem: &sdl2::VideoSubsystem) {
  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(4, 1);
  let (major, minor) = gl_attr.context_version();
  println!("OpenGL version = {}.{}", major, minor);
}

fn rectangle(cx: f32, cy: f32, w: f32, h: f32) -> mesh::Mesh {
  let top_left = Point3 {
    x: cx - w * 0.5,
    y: cy - h * 0.5,
    z: 0.0,
  };
  let top_right = Point3 {
    x: cx + w * 0.5,
    y: cy - h * 0.5,
    z: 0.0,
  };
  let bottom_left = Point3 {
    x: cx - w * 0.5,
    y: cy + h * 0.5,
    z: 0.0,
  };
  let bottom_right = Point3 {
    x: cx + w * 0.5,
    y: cy + h * 0.5,
    z: 0.0,
  };
  let mut mesh = mesh::Mesh::new();
  mesh.clear();
  mesh.add_vertex(top_left.x, top_left.y, top_left.z);
  mesh.add_vertex(top_right.x, top_right.y, top_right.z);
  mesh.add_vertex(bottom_left.x, bottom_left.y, bottom_left.z);

  mesh.add_vertex(bottom_left.x, bottom_left.y, bottom_left.z);
  mesh.add_vertex(top_right.x, top_right.y, top_right.z);
  mesh.add_vertex(bottom_right.x, bottom_right.y, bottom_right.z);
  mesh
}

fn main_sdl() {
  let sdl = sdl2::init().expect("error sdl2 init");
  let video_subsystem = sdl.video().expect("error sdl2 video");
  let window = video_subsystem
    .window("toolkit_test", WINDOW_WIDTH, WINDOW_HEIGHT)
    .opengl()
    .position_centered()
    .resizable()
    .build()
    .expect("error window init");

  setup_gl(&sdl, &video_subsystem);
  let _gl_context = window.gl_create_context().unwrap();
  gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

  let test_mesh = rectangle(0.0, 0.0, 2.0, 2.0);

  let mut yura2_shader =
    shader::Shader::new("assets/shader/thru.vert", "assets/shader/sinwave.frag")
      .expect("fail load yura2_shader");

  let start_time = std::time::Instant::now();
  let mut event_pump = sdl.event_pump().unwrap();
  let mut new_mouse = (0, 0);
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::MouseMotion { x, y, .. } => {
          new_mouse = (x, y);
        }
        _ => (),
      }
    }

    // uniform
    let now_time = std::time::Instant::now();
    let time = (now_time - start_time).as_secs_f32();
    let (res_x, res_y) = window.size();

    unsafe {
      mouse.0 += ((new_mouse.0 - mouse.0) as f32 / 15.0) as i32;
      mouse.1 += ((new_mouse.1 - mouse.1) as f32 / 15.0) as i32;
      freq = 440.0 + 440.0 * (mouse.0 as f32 / WINDOW_WIDTH as f32);
      freqVol = 4.0 + 4.0 * (mouse.1 as f32 / WINDOW_HEIGHT as f32);

      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
      gl::Disable(gl::CULL_FACE);
      gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
      gl::Disable(gl::DEPTH_TEST);

      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      // noise
      yura2_shader.update();
      yura2_shader.begin();
      yura2_shader.set_float(c_str!("iTime"), time);
      yura2_shader.set_vec2(c_str!("iResolution"), res_x as f32, res_y as f32);
      yura2_shader.set_vec2(c_str!("iMouse"), mouse.0 as f32, mouse.1 as f32);
      yura2_shader.set_float(c_str!("iFreq"), freq);
      test_mesh.draw();
    }

    window.gl_swap_window();
  }
  ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
}

//--------------------------------------------------------------------------------
fn main() {
  let handle = std::thread::spawn(move || {
    main_sound().expect("error: main_sound");
  });
  main_sdl();
}
