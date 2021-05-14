use framework::image_manager::ImageManager;
use framework::mesh;
use framework::shader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::path::Path;

use c_str_macro::c_str;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;
fn setupGL(sdl: &sdl2::Sdl, video_subsystem: &sdl2::VideoSubsystem) {
  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(4, 1);
  let (major, minor) = gl_attr.context_version();
  println!("OpenGL version = {}.{}", major, minor);
}

fn Rectangle(cx: f32, cy: f32, w: f32, h: f32) -> mesh::Mesh {
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

fn main() {
  let sdl = sdl2::init().expect("error sdl2 init");
  let video_subsystem = sdl.video().expect("error sdl2 video");
  let window_width = 400;
  let window_height = 400;
  let window = video_subsystem
    .window("toolkit_test", window_width, window_height)
    .opengl()
    .position_centered()
    .resizable()
    .build()
    .expect("error window init");

  setupGL(&sdl, &video_subsystem);
  let _gl_context = window.gl_create_context().unwrap();
  gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

  let test_mesh = Rectangle(0.0, 0.0, 2.0, 2.0);

  let mut tex2d_shader =
    shader::Shader::new("assets/shader/thru.vert", "assets/shader/texture2D.frag").unwrap();

  let mut image_manager = ImageManager::new();
  image_manager.load_image(Path::new("assets/image/surface.png"), "surface", true);

  let surface_texture_id = image_manager.get_texture_id("surface");

  let start_time = std::time::Instant::now();
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
    let now_time = std::time::Instant::now();
    let time = (now_time - start_time).as_secs_f32();
    let (res_x, res_y) = window.size();

    unsafe {
      // gl::Enable(gl::BLEND);
      // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
      gl::Disable(gl::CULL_FACE);
      gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
      gl::Enable(gl::DEPTH_TEST);

      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      tex2d_shader.update();
      tex2d_shader.begin();
      tex2d_shader.set_float(c_str!("iTime"), time);
      tex2d_shader.set_vec2(c_str!("iResolution"), res_x as f32, res_y as f32);
      gl::BindTexture(gl::TEXTURE_2D, surface_texture_id as u32);
      test_mesh.draw();
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    window.gl_swap_window();
  }
  ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
}
