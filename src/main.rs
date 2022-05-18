use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

struct Msg;

struct Model {
    canvas_ref: NodeRef,
    image_ref: NodeRef,
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

    fn rendered(&mut self, _ctx: &Context<Self>, _first_renderr: bool) {
        let window = web_sys::window().unwrap();
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let image = self.image_ref.cast::<HtmlImageElement>().unwrap();

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.set_stroke_style(&"red".into());
        context.stroke_rect(0.0, 0.0, 50.0, 50.0);
        context.set_image_smoothing_enabled(false); // use nearest-neighbour interpolation
        context.draw_image_with_html_image_element(&image, 50.0, 50.0).unwrap();
    }
}

fn main() {
    yew::start_app::<Model>();
}
