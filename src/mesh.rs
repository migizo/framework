pub struct Mesh {
    vertices: Vec<f32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.vertices.clear()
    }
    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(x);
        self.vertices.push(y);
        self.vertices.push(z);
    }
    pub fn draw(&self) {
        unsafe {
            let mut vbo: gl::types::GLuint = 0;
            let mut vao: gl::types::GLuint = 0;

            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (self.vertices.len() * std::mem::size_of::<gl::types::GLfloat>())
                    as gl::types::GLsizeiptr, // size of data in bytes
                self.vertices.as_ptr() as *const std::os::raw::c_void, // pointer to data
                gl::STATIC_DRAW,  // usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::GenVertexArrays(1, &mut vao);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let attribute_type_vec = vec![gl::FLOAT];
            let attribute_size_vec = vec![3];

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                gl::EnableVertexAttribArray(i as u32); // this is "layout (location = 0)" in vertex shader
                gl::VertexAttribPointer(
                    i as u32,              // index of the generic vertex attribute ("layout (location = 0)")
                    attribute_size_vec[i], // the number of components per generic vertex attribute
                    attribute_type_vec[i], // data type
                    gl::FALSE,             // normalized (int-to-float conversion)
                    (3 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizei, // stride (byte offset between consecutive attributes)
                    (offset * std::mem::size_of::<gl::types::GLfloat>())
                        as *const std::os::raw::c_void, // offset of the first component
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            let vertex_num = 2 * 3 as i32;
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_num);
            gl::BindVertexArray(0);
        }
    }
}
