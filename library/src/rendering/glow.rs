// copyright 2023 Remi Bernotavicius

use super::{Color, Event, Renderer};
use alloc::{string::String, vec, vec::Vec};
use glow::HasContext as _;
use std::{io, mem, slice};

unsafe fn compile_shader(
    context: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    let shader = context.create_shader(shader_type)?;
    context.shader_source(shader, source);
    context.compile_shader(shader);

    if context.get_shader_compile_status(shader) {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(shader))
    }
}

unsafe fn link_program(
    context: &glow::Context,
    vert_shader: glow::Shader,
    frag_shader: glow::Shader,
) -> Result<glow::Program, String> {
    let program = context.create_program()?;

    context.attach_shader(program, vert_shader);
    context.attach_shader(program, frag_shader);
    context.link_program(program);

    if context.get_program_link_status(program) {
        Ok(program)
    } else {
        Err(context.get_program_info_log(program))
    }
}

pub struct GlowRenderer {
    texture: glow::Texture,
    program: glow::Program,
    vertex_array: glow::VertexArray,
    front_buffer: Vec<u8>,
    back_buffer: Vec<u8>,
    pub buffer_dirty: bool,
}

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const PIXEL_SIZE: usize = 4;

fn set_rectangle(context: &glow::Context, x: f32, y: f32, width: f32, height: f32) {
    let x1 = x;
    let x2 = x + width;
    let y1 = y;
    let y2 = y + height;
    let data: [f32; 12] = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
    unsafe {
        let u8_slice =
            slice::from_raw_parts(data.as_ptr() as *const u8, 12 * mem::size_of::<f32>());
        context.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_slice, glow::STATIC_DRAW);
    }
}

unsafe fn set_up_context(
    context: &glow::Context,
    texture: glow::Texture,
) -> (glow::Program, glow::VertexArray) {
    let vert_shader = compile_shader(
        context,
        glow::VERTEX_SHADER,
        r#"# version 300 es
        // an attribute is an input (in) to a vertex shader.
        // It will receive data from a buffer
        in vec2 a_position;
        in vec2 a_texCoord;

        // Used to pass in the resolution of the canvas
        uniform vec2 u_resolution;

        // Used to pass the texture coordinates to the fragment shader
        out vec2 v_texCoord;

        // all shaders have a main function
        void main() {

          // convert the position from pixels to 0.0 to 1.0
          vec2 zeroToOne = a_position / u_resolution;

          // convert from 0->1 to 0->2
          vec2 zeroToTwo = zeroToOne * 2.0;

          // convert from 0->2 to -1->+1 (clipspace)
          vec2 clipSpace = zeroToTwo - 1.0;

          gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);

          // pass the texCoord to the fragment shader
          // The GPU will interpolate this value between points.
          v_texCoord = a_texCoord;
        }
        "#,
    )
    .unwrap();
    let frag_shader = compile_shader(
        context,
        glow::FRAGMENT_SHADER,
        r#"# version 300 es
        // fragment shaders don't have a default precision so we need
        // to pick one. highp is a good default. It means "high precision"
        precision highp float;

        // our texture
        uniform sampler2D u_image;

        // the texCoords passed in from the vertex shader.
        in vec2 v_texCoord;

        // we need to declare an output for the fragment shader
        out vec4 outColor;

        void main() {
          outColor = texture(u_image, v_texCoord);
        }
        "#,
    )
    .unwrap();

    let program = link_program(context, vert_shader, frag_shader).unwrap();

    let position_attribute_location: u32 =
        context.get_attrib_location(program, "a_position").unwrap();
    let texcoord_attribute_location: u32 =
        context.get_attrib_location(program, "a_texCoord").unwrap();

    let resolution_location = context
        .get_uniform_location(program, "u_resolution")
        .unwrap();
    let image_location = context.get_uniform_location(program, "u_image").unwrap();

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(vao));

    let position_buffer = context.create_buffer().unwrap();
    context.enable_vertex_attrib_array(position_attribute_location);
    context.bind_buffer(glow::ARRAY_BUFFER, Some(position_buffer));
    context.vertex_attrib_pointer_f32(position_attribute_location, 2, glow::FLOAT, false, 0, 0);
    let texcoord_buffer = context.create_buffer().unwrap();
    context.bind_buffer(glow::ARRAY_BUFFER, Some(texcoord_buffer));

    let data: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
    let u8_slice = slice::from_raw_parts(data.as_ptr() as *const u8, 12 * mem::size_of::<f32>());
    context.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_slice, glow::STATIC_DRAW);

    context.enable_vertex_attrib_array(texcoord_attribute_location);

    context.vertex_attrib_pointer_f32(texcoord_attribute_location, 2, glow::FLOAT, false, 0, 0);

    context.active_texture(glow::TEXTURE0);
    context.bind_texture(glow::TEXTURE_2D, Some(texture));

    context.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::NEAREST as i32,
    );
    context.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::NEAREST as i32,
    );

    context.viewport(
        0,
        0,
        (WIDTH * PIXEL_SIZE) as i32,
        (HEIGHT * PIXEL_SIZE) as i32,
    );
    context.use_program(Some(program));

    let width = (WIDTH * PIXEL_SIZE) as f32;
    let height = (HEIGHT * PIXEL_SIZE) as f32;

    context.uniform_2_f32(Some(&resolution_location), width, height);
    context.uniform_1_i32(Some(&image_location), 0);

    context.bind_buffer(glow::ARRAY_BUFFER, Some(position_buffer));
    set_rectangle(context, 0.0, 0.0, width, height);

    (program, vao)
}

impl GlowRenderer {
    pub fn new(context: &glow::Context) -> Self {
        let texture = unsafe { context.create_texture() }.unwrap();
        let (program, vertex_array) = unsafe { set_up_context(context, texture) };
        Self {
            texture,
            program,
            vertex_array,
            front_buffer: vec![u8::MAX; WIDTH * HEIGHT * 4],
            back_buffer: vec![u8::MAX; WIDTH * HEIGHT * 4],
            buffer_dirty: false,
        }
    }

    pub fn render(&mut self, context: &glow::Context) {
        unsafe {
            context.bind_vertex_array(Some(self.vertex_array));
            context.active_texture(glow::TEXTURE0);
            context.use_program(Some(self.program));
            context.bind_texture(glow::TEXTURE_2D, Some(self.texture));

            if self.buffer_dirty {
                context.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    WIDTH as i32,
                    HEIGHT as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    Some(&self.front_buffer[..]),
                );
                self.buffer_dirty = false;
            }
            context.draw_arrays(glow::TRIANGLES, 0, 6)
        }
    }
}

#[derive(Clone)]
pub struct SimpleColor {
    r: u8,
    g: u8,
    b: u8,
}

impl Color for SimpleColor {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Renderer for GlowRenderer {
    type Color = SimpleColor;

    fn poll_events(&mut self) -> Vec<Event> {
        vec![]
    }

    fn save_buffer(&self, _: impl io::Write) -> io::Result<()> {
        unimplemented!()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    fn color_pixel(&mut self, x: i32, y: i32, color: Self::Color) {
        assert!(x < WIDTH as i32, "{}", "x = {x} > {WIDTH}");
        assert!(y < HEIGHT as i32, "{}", "y = {y} > {HEIGHT}");
        assert!(x >= 0, "{}", "x = {x} > 0");
        assert!(y >= 0, "{}", "y = {y} > 0");

        let i = (y as usize * WIDTH + x as usize) * 4;
        self.back_buffer[i] = color.r;
        self.back_buffer[i + 1] = color.g;
        self.back_buffer[i + 2] = color.b;
        self.back_buffer[i + 3] = 255;
    }

    fn present(&mut self) {
        mem::swap(&mut self.front_buffer, &mut self.back_buffer);
        self.buffer_dirty = true;
    }
}
