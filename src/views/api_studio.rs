use gpui::{Window, div, prelude::*, px, rgb, rgba, size};

pub struct ApiStudio;

impl Render for ApiStudio {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x18181be6))
            .text_color(rgb(0xffffff))
            .text_xl()
            .child("Rust API Studio")
    }
}
