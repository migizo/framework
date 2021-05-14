use cgmath::Array;
use cgmath::Matrix;
use std::ffi::{CStr, CString};

#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

// auto update
use std::fs;
use std::time::Duration;
use std::time::SystemTime;

use std::fs::File;
use std::io::prelude::*;

pub struct Shader {
  program_id: gl::types::GLuint,
  vert_filename: &'static str,
  frag_filename: &'static str,
  last_vert_mod_time: SystemTime,
  last_frag_mod_time: SystemTime,
  last_check_time: SystemTime,
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.program_id);
    }
  }
}

impl Shader {
  pub fn new(vert_filename: &'static str, frag_filename: &'static str) -> Result<Shader, String> {
    let last_vert_mod_time = fs::metadata(vert_filename)
      .expect("last_vert_mod_time is not founded")
      .modified()
      .unwrap();
    let last_frag_mod_time = fs::metadata(frag_filename)
      .expect("last_frag_mod_time is not founded")
      .modified()
      .unwrap();
    let last_check_time = SystemTime::now();

    let program_id = Shader::load(vert_filename, frag_filename)?;

    Ok(Shader {
      program_id,
      vert_filename,
      frag_filename,
      last_vert_mod_time,
      last_frag_mod_time,
      last_check_time,
    })
  }

  fn load(
    vert_filename: &'static str,
    frag_filename: &'static str,
  ) -> Result<gl::types::GLuint, String> {
    // vert
    let mut v_file = File::open(vert_filename).unwrap();
    let mut v_source = String::new();
    v_file.read_to_string(&mut v_source).unwrap();
    let vert_shader =
      Shader::shader_from_source(&CString::new(v_source).unwrap(), gl::VERTEX_SHADER)?;

    // frag
    let mut f_file = File::open(frag_filename).unwrap();
    let mut f_source = String::new();
    f_file.read_to_string(&mut f_source).unwrap();
    let frag_shader =
      Shader::shader_from_source(&CString::new(f_source).unwrap(), gl::FRAGMENT_SHADER)?;

    let program_id = Shader::program_from_shaders(&[vert_shader, frag_shader])?;

    unsafe {
      gl::DeleteShader(vert_shader);
      gl::DeleteShader(frag_shader);
    }

    Ok(program_id)
  }

  pub fn update(&mut self) {
    if SystemTime::now()
      .duration_since(self.last_check_time)
      .unwrap()
      .as_secs()
      > 1
    {
      self.last_check_time = SystemTime::now();

      // check mod time(vert)
      if fs::metadata(self.vert_filename)
        .unwrap()
        .modified()
        .unwrap()
        != self.last_vert_mod_time
      {
        self.last_vert_mod_time = fs::metadata(self.vert_filename)
          .unwrap()
          .modified()
          .unwrap();
        println!("modified! {}", self.vert_filename);
        self.program_id = Shader::load(self.vert_filename, self.frag_filename).unwrap();
      }

      // check mod time(frag)
      if fs::metadata(self.frag_filename)
        .unwrap()
        .modified()
        .unwrap()
        != self.last_frag_mod_time
      {
        self.last_frag_mod_time = fs::metadata(self.frag_filename)
          .unwrap()
          .modified()
          .unwrap();
        println!("modified! {}", self.frag_filename);
        self.program_id = Shader::load(self.vert_filename, self.frag_filename).unwrap();
      }
    }
  }

  pub unsafe fn begin(&self) {
    gl::UseProgram(self.program_id);
  }
  pub unsafe fn end(&self) {}

  pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
    gl::Uniform1i(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      value as i32,
    );
  }

  pub unsafe fn set_int(&self, name: &CStr, value: i32) {
    gl::Uniform1i(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      value,
    );
  }

  pub unsafe fn set_float(&self, name: &CStr, value: f32) {
    gl::Uniform1f(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      value,
    );
  }

  pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3) {
    gl::Uniform3fv(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      1,
      value.as_ptr(),
    );
  }

  pub unsafe fn set_vec2(&self, name: &CStr, x: f32, y: f32) {
    gl::Uniform2f(gl::GetUniformLocation(self.program_id, name.as_ptr()), x, y);
  }

  pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
    gl::Uniform3f(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      x,
      y,
      z,
    );
  }
  pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4) {
    gl::UniformMatrix4fv(
      gl::GetUniformLocation(self.program_id, name.as_ptr()),
      1,
      gl::FALSE,
      mat.as_ptr(),
    );
  }

  fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLenum,
  ) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
      gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
      gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
      let mut len: gl::types::GLint = 0;
      unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring_with_len(len as usize);

      unsafe {
        gl::GetShaderInfoLog(
          id,
          len,
          std::ptr::null_mut(),
          error.as_ptr() as *mut gl::types::GLchar,
        );
      }

      return Err(error.to_string_lossy().into_owned());
    }

    Ok(id as gl::types::GLuint)
  }

  pub fn program_from_shaders(
    shader_ids: &[gl::types::GLuint],
  ) -> Result<gl::types::GLuint, String> {
    let program_id = unsafe { gl::CreateProgram() };

    for shader_id in shader_ids {
      unsafe {
        gl::AttachShader(program_id, *shader_id);
      }
    }

    unsafe {
      gl::LinkProgram(program_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
      gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
      let mut len: gl::types::GLint = 0;
      unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
      }

      let error = create_whitespace_cstring_with_len(len as usize);

      unsafe {
        gl::GetProgramInfoLog(
          program_id,
          len,
          std::ptr::null_mut(),
          error.as_ptr() as *mut gl::types::GLchar,
        );
      }

      return Err(error.to_string_lossy().into_owned());
    }

    for shader_id in shader_ids {
      unsafe {
        gl::DetachShader(program_id, *shader_id);
      }
    }

    Ok(program_id)
  }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
  // allocate buffer of correct size
  let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
  // fill it with len spaces
  buffer.extend([b' '].iter().cycle().take(len));
  // convert buffer to CString
  unsafe { CString::from_vec_unchecked(buffer) }
}
