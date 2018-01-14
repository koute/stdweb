#[macro_use]
extern crate stdweb;

use stdweb::Reference;
use stdweb::unstable::TryInto;
use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    document,
    window,
    RenderingContext
};

use stdweb::web::event::{
    IMouseEvent,
    MouseMoveEvent,
    ResizeEvent,
};

use stdweb::web::html_element::CanvasElement;

// Shamelessly stolen from webplatform's TodoMVC example.
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

struct CanvasRenderingContext2D(Reference);

reference_boilerplate! {
    CanvasRenderingContext2D,
    instanceof CanvasRenderingContext2D
}

impl RenderingContext for CanvasRenderingContext2D {
    const CONTEXT_TYPE: &'static str = "2d";
}

impl CanvasRenderingContext2D {
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        js! { @(no_return)
            @{self}.fillRect(@{x}, @{y}, @{width}, @{height});
        }
    }
}

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().try_into().unwrap();
    let context: CanvasRenderingContext2D = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    canvas.add_event_listener( enclose!( (context) move |event: MouseMoveEvent| {
        context.fill_rect(event.client_x() - 5.0, event.client_y() - 5.0, 10.0, 10.0);
    }));

    stdweb::event_loop();
}
