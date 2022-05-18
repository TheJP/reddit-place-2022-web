use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

struct Msg;

struct Model {
    canvas_ref: NodeRef,
    image_ref: NodeRef,
}

impl Model {
    fn resize_canvas(canvas_ref: &NodeRef) {
        let window = web_sys::window().expect("failed to get `window`");
        let canvas = canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("could not find canvas element");

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    }

    fn draw_image(canvas_ref: &NodeRef, image_ref: &NodeRef) {
        let canvas = canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("could not find canvas element");
        let image = image_ref
            .cast::<HtmlImageElement>()
            .expect("could not find img element");

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.set_image_smoothing_enabled(false); // use nearest-neighbour interpolation
        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
            .expect("failed to draw image");
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_ref: NodeRef::default(),
            image_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
        html! {
            <div>
                <canvas ref={self.canvas_ref.clone()} />
                <img ref={self.image_ref.clone()} src="images/final_clean.png" style="display: none" />
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let window = web_sys::window().expect("failed to get `window`");

        // Resize canvas once in the beginning.
        Self::resize_canvas(&self.canvas_ref);
        Self::draw_image(&self.canvas_ref, &self.image_ref);

        // Setup event to resize the canvas every time the window gets resized.
        let closure = {
            let canvas_ref = self.canvas_ref.clone();
            let image_ref = self.image_ref.clone();
            move || {
                Self::resize_canvas(&canvas_ref);
                Self::draw_image(&canvas_ref, &image_ref);
            }
        };
        let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);
        window.set_onresize(Some(closure.as_ref().unchecked_ref()));

        // Makes sure that closure is not dropped at the end of this function.
        // Because this would make resize events fail on something similar to a dangling pointer.
        closure.forget();
    }
}

fn main() {
    yew::start_app::<Model>();
}
