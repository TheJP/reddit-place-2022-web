use web_sys::{MouseEvent, WheelEvent};

#[derive(Copy, Clone)]
pub struct MousePosition(pub i32, pub i32);

impl From<MouseEvent> for MousePosition {
    fn from(mouse_event: MouseEvent) -> Self {
        Self(mouse_event.page_x(), mouse_event.page_y())
    }
}

impl From<WheelEvent> for MousePosition {
    fn from(wheel_event: WheelEvent) -> Self {
        Self(wheel_event.page_x(), wheel_event.page_y())
    }
}
