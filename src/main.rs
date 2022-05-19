mod mouse_position;

use mouse_position::MousePosition;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::prelude::*;

const ZOOM_INTENSITY: f64 = 1.2;

enum Msg {
    MouseDown(i16, MousePosition),
    MouseUp(i16, MousePosition),
    MouseMove(MousePosition),
    Wheel(f64, MousePosition),
    Resize,
}

struct Model {
    canvas_ref: NodeRef,
    image_ref: NodeRef,
}

impl Model {
    fn canvas(&self) -> (HtmlCanvasElement, CanvasRenderingContext2d) {
        let canvas = self
            .canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("could not find canvas element");
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        (canvas, context)
    }

    fn resize_canvas(&self) {
        let window = web_sys::window().expect("failed to get `window`");
        let canvas = self
            .canvas_ref
            .cast::<HtmlCanvasElement>()
            .expect("could not find canvas element");

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    }

    fn draw_image(&self) {
        let image = self
            .image_ref
            .cast::<HtmlImageElement>()
            .expect("could not find img element");
        let (canvas, context) = self.canvas();

        context.save();
        context
            .reset_transform()
            .expect("could not reset canvas transform");
        context.set_fill_style(&"black".into());
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.restore();

        context.set_image_smoothing_enabled(false); // use nearest-neighbour interpolation
        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
            .expect("failed to draw image");
    }

    fn draw(&self) {
        self.draw_image();
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => {
                self.resize_canvas();
                self.draw();
            }
            Msg::MouseDown(button, _) if button == 0 => {
                let (_, context) = self.canvas();
                context.scale(5.0, 5.0).unwrap();
                self.draw();
            }
            Msg::Wheel(delta, _) => {
                let (_, context) = self.canvas();
                let delta = if delta < 0.0 {
                    ZOOM_INTENSITY
                } else {
                    1.0 / ZOOM_INTENSITY
                };
                context.scale(delta, delta).unwrap();
                self.draw();
            }
            Msg::MouseMove(position) => {
                self.draw();
                let (_, context) = self.canvas();
                context.set_fill_style(&"red".into());
                context.fill_rect(position.x as f64, position.y as f64, 50.0, 50.0);
            }
            _ => {}
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx
            .link()
            .callback(|m: MouseEvent| Msg::MouseDown(m.button(), m.into()));
        let onmouseup = ctx
            .link()
            .callback(|m: MouseEvent| Msg::MouseUp(m.button(), m.into()));
        let onmousemove = ctx
            .link()
            .callback(|m: MouseEvent| Msg::MouseMove(m.into()));

        // Prevents context menu from appearing.
        let oncontextmenu = ctx.link().batch_callback(|m: MouseEvent| {
            m.prevent_default();
            None
        });

        let onwheel = ctx
            .link()
            .callback(|w: WheelEvent| Msg::Wheel(w.delta_y(), w.into()));

        // Makes canvas resize and redraw as soon as the image is loaded.
        let onload = ctx.link().callback(|_| Msg::Resize);

        html! {
            <div>
                <canvas {onmousedown} {onmouseup} {onmousemove} {oncontextmenu} {onwheel} ref={self.canvas_ref.clone()} />
                <img {onload} ref={self.image_ref.clone()} src="images/final_clean.png" style="display: none" />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let window = web_sys::window().expect("failed to get `window`");

        // Resize and draw canvas once in the beginning.
        self.resize_canvas();
        self.draw();

        // Setup event to resize and redraw the canvas every time the window gets resized.
        let onresize = ctx.link().callback(|_: Event| Msg::Resize);
        let closure = move |e| onresize.emit(e);
        let closure = Closure::wrap(Box::new(closure) as Box<dyn Fn(Event)>);
        window.set_onresize(Some(closure.as_ref().unchecked_ref()));

        // Makes sure that closure is not dropped at the end of this function.
        // Because this would make resize events fail on something similar to a dangling pointer.
        closure.forget();
    }
}

fn main() {
    yew::start_app::<Model>();
}
