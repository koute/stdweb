#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;

use std::rc::Rc;
use std::cell::RefCell;

mod webgl_rendering_context;

use stdweb::unstable::TryInto;
use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    document,
    window,
    TypedArray,
};

use stdweb::web::event::{
    ResizeEvent,
};

use stdweb::web::html_element::CanvasElement;
use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLBuffer
};

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

/*==================== Rotation ====================*/
fn rotate_z(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0] = c*m[0]-s*m[1];
    m[4] = c*m[4]-s*m[5];
    m[8] = c*m[8]-s*m[9];

    m[1]=c*m[1]+s*mv0;
    m[5]=c*m[5]+s*mv4;
    m[9]=c*m[9]+s*mv8;
}

fn rotate_x(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv1, mv5, mv9) = (m[1], m[5], m[9]);

    m[1] = m[1]*c-m[2]*s;
    m[5] = m[5]*c-m[6]*s;
    m[9] = m[9]*c-m[10]*s;

    m[2] = m[2]*c+mv1*s;
    m[6] = m[6]*c+mv5*s;
    m[10] = m[10]*c+mv9*s;
}

fn rotate_y(m: &mut [f32], angle: f32) {
    let (s, c) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0] = c*m[0]+s*m[2];
    m[4] = c*m[4]+s*m[6];
    m[8] = c*m[8]+s*m[10];

    m[2] = c*m[2]-s*mv0;
    m[6] = c*m[6]-s*mv4;
    m[10] = c*m[10]-s*mv8;
}

/*==================== MATRIX =====================*/
fn get_projection(angle: f32, a: f32, z_min: f32, z_max: f32) -> [f32; 16] {
    let ang = (angle*0.5).to_radians().tan();
    return [
        0.5/ang, 0., 0., 0.,
        0., 0.5*a/ang, 0., 0.,
        0., 0., -(z_max+z_min)/(z_max-z_min), -1.,
        0., 0., (-2.*z_max*z_min)/(z_max-z_min), 0.
    ];
}

struct State {
    time_old: f64,
    mov_matrix: [f32; 16],
    view_matrix: [f32; 16],
    canvas: CanvasElement,
    context: gl,
    p_matrix: WebGLUniformLocation,
    v_matrix: WebGLUniformLocation,
    m_matrix: WebGLUniformLocation,
    index_buffer: WebGLBuffer,
}

impl State {
    fn animate(&mut self, time: f64, rc: Rc<RefCell<Self>>) {
        let dt = (time - self.time_old) as f32;
        rotate_z(&mut self.mov_matrix, dt*0.0007);//time
        rotate_y(&mut self.mov_matrix, dt*0.0002);
        rotate_x(&mut self.mov_matrix, dt*0.0003);
        self.time_old = time;

        self.context.enable(gl::DEPTH_TEST);
        self.context.depth_func(gl::LEQUAL);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        self.context.clear_depth(1.0);

        let (w, h) = (self.canvas.width(), self.canvas.height());
        let proj_matrix = get_projection(40., (w as f32)/(h as f32), 1., 100.);

        self.context.viewport(0, 0, w as i32, h as i32);
        self.context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.context.uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..]);
        self.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        self.context.draw_elements(gl::TRIANGLES, 36, gl::UNSIGNED_SHORT, 0);

        window().request_animation_frame(move |time| {
            rc.borrow_mut().animate(time, rc.clone());
        });
    }
}

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().try_into().unwrap();
    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    context.clear_color(1.0, 0.0, 0.0, 1.0);
    context.clear(gl::COLOR_BUFFER_BIT);

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    let vertices = TypedArray::<f32>::from(&[
        -1.,-1.,-1.,  1.,-1.,-1.,  1., 1.,-1., -1., 1.,-1.,
        -1.,-1., 1.,  1.,-1., 1.,  1., 1., 1., -1., 1., 1.,
        -1.,-1.,-1., -1., 1.,-1., -1., 1., 1., -1.,-1., 1.,
         1.,-1.,-1.,  1., 1.,-1.,  1., 1., 1.,  1.,-1., 1.,
        -1.,-1.,-1., -1.,-1., 1.,  1.,-1., 1.,  1.,-1.,-1.,
        -1., 1.,-1., -1., 1., 1.,  1., 1., 1.,  1., 1.,-1., 
    ][..]).buffer();

    let colors = TypedArray::<f32>::from(&[
        5.,3.,7., 5.,3.,7., 5.,3.,7., 5.,3.,7.,
        1.,1.,3., 1.,1.,3., 1.,1.,3., 1.,1.,3.,
        0.,0.,1., 0.,0.,1., 0.,0.,1., 0.,0.,1.,
        1.,0.,0., 1.,0.,0., 1.,0.,0., 1.,0.,0.,
        1.,1.,0., 1.,1.,0., 1.,1.,0., 1.,1.,0.,
        0.,1.,0., 0.,1.,0., 0.,1.,0., 0.,1.,0.
    ][..]).buffer();

    let indices = TypedArray::<u16>::from(&[
        0,1,2, 0,2,3, 4,5,6, 4,6,7,
        8,9,10, 8,10,11, 12,13,14, 12,14,15,
        16,17,18, 16,18,19, 20,21,22, 20,22,23 
    ][..]).buffer();

    // Create and store data into vertex buffer
    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

    // Create and store data into color buffer
    let color_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

    // Create and store data into index buffer
    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

    /*=================== Shaders =========================*/
    let vert_code = r#"
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;
        attribute vec3 color;
        varying vec3 vColor;

        void main() {
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
            vColor = color;
        }
    "#;

    let frag_code = r#"
        precision mediump float;
        varying vec3 vColor;

        void main() {
            gl_FragColor = vec4(vColor, 1.);
        }
    "#;

    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, vert_code);
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, frag_code);
    context.compile_shader(&frag_shader);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    /* ====== Associating attributes to vertex shader =====*/
    let p_matrix = context.get_uniform_location(&shader_program, "Pmatrix").unwrap();
    let v_matrix = context.get_uniform_location(&shader_program, "Vmatrix").unwrap();
    let m_matrix = context.get_uniform_location(&shader_program, "Mmatrix").unwrap();

    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    let position = context.get_attrib_location(&shader_program, "position") as u32;
    context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0) ;

    // Position
    context.enable_vertex_attrib_array(position);
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    let color = context.get_attrib_location(&shader_program, "color") as u32;
    context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0) ;

    // Color
    context.enable_vertex_attrib_array(color);
    context.use_program(Some(&shader_program));

    let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
    let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

    // translating z
    view_matrix[14] -= 6.; //zoom

    let state = Rc::new(RefCell::new(State {
        time_old: 0.0,
        mov_matrix,
        view_matrix,
        canvas,
        context,
        p_matrix,
        v_matrix,
        m_matrix,
        index_buffer,
    }));

    state.borrow_mut().animate(0., state.clone());

    stdweb::event_loop();
}
