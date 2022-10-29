// copyright 2021 Remi Bernotavicius

use come_boy::rendering::{Color, Event, Keycode, Renderer};
use std::convert::TryInto;
use std::{io, mem};
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTexture};

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub struct CanvasRenderer {
    context: WebGl2RenderingContext,
    texture: WebGlTexture,
    buffer: Vec<u8>,
    keyboard_events: Vec<Event>,
}

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const PIXEL_SIZE: usize = 4;

fn get_rendering_context(canvas: &web_sys::HtmlCanvasElement) -> WebGl2RenderingContext {
    canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap()
}

fn set_rectangle(context: &WebGl2RenderingContext, x: f32, y: f32, width: f32, height: f32) {
    let x1 = x;
    let x2 = x + width;
    let y1 = y;
    let y2 = y + height;
    let data: [f32; 12] = [x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2];
    unsafe {
        let data_array = js_sys::Float32Array::view(&data);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &data_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

fn set_up_context(context: &WebGl2RenderingContext, texture: &WebGlTexture) {
    let vert_shader = compile_shader(
        context,
        WebGl2RenderingContext::VERTEX_SHADER,
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
        WebGl2RenderingContext::FRAGMENT_SHADER,
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

    let program = link_program(context, &vert_shader, &frag_shader).unwrap();

    let position_attribute_location: u32 = context
        .get_attrib_location(&program, "a_position")
        .try_into()
        .unwrap();
    let texcoord_attribute_location: u32 = context
        .get_attrib_location(&program, "a_texCoord")
        .try_into()
        .unwrap();

    let resolution_location = context
        .get_uniform_location(&program, "u_resolution")
        .unwrap();
    let image_location = context.get_uniform_location(&program, "u_image").unwrap();

    let vao = context.create_vertex_array().unwrap();
    context.bind_vertex_array(Some(&vao));

    let position_buffer = context.create_buffer().unwrap();
    context.enable_vertex_attrib_array(position_attribute_location);
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    let texcoord_buffer = context.create_buffer().unwrap();
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&texcoord_buffer));

    let data: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];
    unsafe {
        let data_array = js_sys::Float32Array::view(&data);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &data_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.enable_vertex_attrib_array(texcoord_attribute_location);

    context.vertex_attrib_pointer_with_i32(
        texcoord_attribute_location,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.active_texture(WebGl2RenderingContext::TEXTURE0);
    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(texture));

    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_S,
        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_T,
        WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );

    context.viewport(
        0,
        0,
        (WIDTH * PIXEL_SIZE) as i32,
        (HEIGHT * PIXEL_SIZE) as i32,
    );
    context.use_program(Some(&program));

    let width = (WIDTH * PIXEL_SIZE) as f32;
    let height = (HEIGHT * PIXEL_SIZE) as f32;

    context.uniform2f(Some(&resolution_location), width, height);
    context.uniform1i(Some(&image_location), 0);

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
    set_rectangle(context, 0.0, 0.0, width, height);
}

fn keycode_from_native_code(code: &str) -> Keycode {
    match code {
        "ArrowDown" => Keycode::Down,
        "ArrowLeft" => Keycode::Left,
        "ArrowRight" => Keycode::Right,
        "ArrowUp" => Keycode::Up,
        "Tab" => Keycode::Tab,
        "KeyX" => Keycode::X,
        "KeyZ" => Keycode::Z,
        "Enter" => Keycode::Return,
        "F2" => Keycode::F2,
        "F3" => Keycode::F3,
        "F4" => Keycode::F4,
        _ => Keycode::Unknown,
    }
}

impl CanvasRenderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let context = get_rendering_context(canvas);
        let texture = context.create_texture().unwrap();
        set_up_context(&context, &texture);
        Self {
            context,
            texture,
            buffer: vec![u8::MAX; WIDTH * HEIGHT * 4],
            keyboard_events: vec![],
        }
    }

    pub fn render(&self) {
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }

    pub fn on_key_down(&mut self, code: &str) {
        self.keyboard_events
            .push(Event::KeyDown(keycode_from_native_code(code)));
    }

    pub fn on_key_up(&mut self, code: &str) {
        self.keyboard_events
            .push(Event::KeyUp(keycode_from_native_code(code)));
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

impl Renderer for CanvasRenderer {
    type Color = SimpleColor;

    fn poll_events(&mut self) -> Vec<Event> {
        mem::take(&mut self.keyboard_events)
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
        self.buffer[i] = color.r;
        self.buffer[i + 1] = color.g;
        self.buffer[i + 2] = color.b;
        self.buffer[i + 3] = 255;
    }

    fn present(&mut self) {
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));

        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WIDTH as i32,
                HEIGHT as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&self.buffer[..]),
            )
            .unwrap();
    }
}
