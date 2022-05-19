use web_sys::{MouseEvent, WheelEvent};

pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

impl From<MouseEvent> for MousePosition {
    fn from(mouse_event: MouseEvent) -> Self {
        Self {
            x: mouse_event.page_x(),
            y: mouse_event.page_y(),
        }
    }
}

impl From<WheelEvent> for MousePosition {
    fn from(wheel_event: WheelEvent) -> Self {
        Self {
            x: wheel_event.page_x(),
            y: wheel_event.page_y(),
        }
    }
}
