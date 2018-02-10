extern crate stdweb;

use stdweb::unstable::TryInto;
use stdweb::web::{
    IParentNode,
    IHtmlElement,
    document,
    WebGLRenderingContext
};

use stdweb::web::html_element::CanvasElement;

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
    let context: WebGLRenderingContext = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    context.clear_color(0.7, 0.2, 0.5, 1.0);
    context.clear();

    stdweb::event_loop();
}
