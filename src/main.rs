#[macro_use]
extern crate glium;
use glium::{Surface, glutin};

extern crate glium_text_rusttype as text;

extern crate cgmath;

use std::path::Path;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    println!("Hello, world!");

    let w = 500;
    let h = 500;

    let mut event_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
    										.with_dimensions(w, h)
    										.with_title("hello");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut closed = false;

    let v1 = Vertex {position: [0.0, 0.0]};
    let v2 = Vertex {position: [0.0, 0.5]};
    let v3 = Vertex {position: [0.5, 0.5]};

    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    	#version 330

    	in vec2 position;
    	uniform float t;

    	void main() {
    		vec2 pos = position;
    		pos.x += t + 0.1*sin(t);
    		gl_Position = vec4(pos, 0.0, 1.0);
    	}
    "#;

    let fragment_shader_src = r#"
    	#version 330
    	out vec4 color;

    	void main() {
    		color = vec4(1.0, 0.0, 0.0, 1.0);
    	}
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;

    let text_system = text::TextSystem::new(&display);
    let font_file = File::open(&Path::new(&"Hack-Regular.ttf")).expect(".ttf file not found");
    let font = text::FontTexture::new(&display,
    								  font_file,
    								  70,
    								  text::FontTexture::ascii_character_list()).unwrap();

    let mut text_buffer = String::new();

    // each line is actualy a column (column major)
    let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
            0.1, 0.0, 0.0, 0.0,
            0.0, 0.1 * ((w as f32)/(h as f32)) , 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -0.9, 0.8, 0.0, 1.0f32,
    	).into();

    while !closed {
    	t+= 0.001;
    	if t > 1.0 {
    		t = -1.0;
    	}

    	let mut target = display.draw();

	    	target.clear_color(0.0, 1.0, 0.1, 1.0);

	    	target.draw(&vertex_buffer,
	    				&indices,
	    				&program,
	    				&uniform!{t: t},
	    				&Default::default()).unwrap();	    	

	    	let text = text::TextDisplay::new(&text_system, &font, &text_buffer);

	    	text::draw(&text, &text_system, &mut target, matrix, (1.0,0.0,0.0,1.0)).unwrap();

    	target.finish().unwrap();



    	event_loop.poll_events(|ev| {
    		match ev {
    			glutin::Event::WindowEvent {event, ..} => match event {
    				glutin::WindowEvent::Closed => closed = true,
    				glutin::WindowEvent::KeyboardInput {input, ..} => {
    					match input.virtual_keycode {
    						Some (keycode) => {
    							match keycode {
    								glutin::VirtualKeyCode::Escape => closed = true,
    								glutin::VirtualKeyCode::Back => {
    									text_buffer.pop();
    									()
    								},
    								_ => (),
    							}
    						}
    						_ => (),
    					}
    				}
    				glutin::WindowEvent::ReceivedCharacter('\r') => text_buffer.clear(),
    				glutin::WindowEvent::ReceivedCharacter('\n') => text_buffer.clear(),
    				glutin::WindowEvent::ReceivedCharacter(chr) => {
						text_buffer.push(chr);
						print!("{:?}", chr);
    				},
    				_ => (),
    			},
    			_ => (),

    		}
    	});
    }
}
