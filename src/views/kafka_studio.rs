use gpui::{Div, Window, div, prelude::*, px, rgb, rgba};

#[derive(Debug)]
pub struct KafkaStudio {
    selected_connection: Option<usize>,
}

impl Default for KafkaStudio {
    fn default() -> Self {
        Self {
            selected_connection: None,
        }
    }
}

impl Render for KafkaStudio {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgba(0x18181be6))
            .text_color(rgb(0xffffff))
            .child(top_menu())
            .child(
                div()
                    .flex()
                    .flex_1()
                    .text_sm()
                    .child(connection_list(cx, self.selected_connection))
                    .child(connection_workspace(self.selected_connection)),
            )
    }
}

pub(super) fn top_menu() -> Div {
    div()
        .flex()
        .items_center()
        .px_4()
        .h(px(48.0))
        .text_lg()
        .child("Rust Kafka Studio")
}

pub(super) fn connection_list(
    cx: &mut Context<KafkaStudio>,
    selected_connection: Option<usize>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .w(px(240.0))
        .flex_shrink_0() // 空间不足时不压缩侧栏
        .p_6()
        .border_r_1()
        .border_color(rgb(0x475569))
        .child("Connections (Demo)")
        .child(
            div()
                .id("Local Demo")
                .bg(if selected_connection == Some(0) {
                    rgba(0x334155e6)
                } else {
                    rgba(0x18181be6)
                })
                .p_2()
                .cursor_pointer()
                .child("GET Example")
                .on_click(cx.listener(|this, _event, _window, cx| {
                    this.selected_connection = Some(0);
                    cx.notify();
                })),
        )
        .child(
            div()
                .id("Test Demo")
                .bg(if selected_connection == Some(1) {
                    rgba(0x334155e6)
                } else {
                    rgba(0x18181be6)
                })
                .p_2()
                .cursor_pointer()
                .child("POST Example")
                .on_click(cx.listener(|this, _event, _window, cx| {
                    this.selected_connection = Some(1);
                    cx.notify();
                })),
        )
}

pub(super) fn connection_workspace(selected_connection: Option<usize>) -> Div {
    let text = match selected_connection {
        None => String::from("Select a connection"),
        Some(index) => format!("Selected demo connection #{index} — not connected"),
    };
    div().flex().flex_1().p_6().child(text)
}
