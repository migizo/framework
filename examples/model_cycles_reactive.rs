// midir
use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, Ignore};

#[derive(Default, Clone, Copy)]
struct Track {
  val: [u8; 2]
}
impl Track {
  fn new(val: [u8; 2]) -> Self{
    Self{val}
  }
}
static mut MIDI_TRACK: [Option<Track>; 6] = [None; 6];

//--------------------------------------------------------------------------------
//sdl
use std::time::Duration;

use framework::mesh;
use framework::shader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use c_str_macro::c_str;
use std::ffi::CString; // NULL終端文字列
use cgmath::prelude::SquareMatrix;

static mut mouse: (i32, i32) = (0, 0);
static mut freq: f32 = 0.0;
static mut freqVol: f32 = 0.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 800;

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

  let mut shader =
    shader::Shader::new("assets/shader/thru.vert", "assets/shader/modelCyclesReactive.frag")
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
      shader.update();
      shader.begin();
      shader.set_float(c_str!("iTime"), time);
      shader.set_vec2(c_str!("iResolution"), res_x as f32, res_y as f32);
      shader.set_float(c_str!("iFreq"), freq);
      for i in 0..6 {
        let t_name = "T[".to_string() + &i.to_string() + "]";
        let t_name = CString::new(t_name).unwrap();
        shader.set_float(&t_name, MIDI_TRACK[i].unwrap().val[1] as f32 / 255.0);
      }
      // shader.set_float(c_str!("T[0]"), 0.0);
      // shader.set_float(c_str!("T[1]"), 0.0);
      // shader.set_float(c_str!("T[2]"), 0.0);
      // shader.set_float(c_str!("T[3]"), 0.0);
      // shader.set_float(c_str!("T[4]"), 0.0);
      // shader.set_float(c_str!("T[5]"), 0.0);

      test_mesh.draw();
    }

    window.gl_swap_window();
  }
  ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

fn main() {
  unsafe {
    MIDI_TRACK = [Some(Track::default()); 6];
  }

  let midi_handle = std::thread::spawn(move || {
    match run() {
      Ok(_) => (),
      Err(err) => println!("Error: {}", err)
    }
  });

  main_sdl();
}




fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!("Choosing the only available input port: {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid input port selected")?
        }
    };
    
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        // println!("{}: {:?} (len = {})", stamp, message, message.len());
        if message.len() == 3 {
          let m_ch = message[0] - 144;
          if m_ch < 6 {
            unsafe {
              MIDI_TRACK[m_ch as usize] = Some(Track::new([message[1], message[2]]));
            }
          }
        }
    }, ())?;
    
    println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
