use gpui::{px, size, App, Application, Bounds, WindowBounds, WindowOptions};
use gpui::{AppContext, WindowBackgroundAppearance};
use kafka_studio::views;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |_, cx| cx.new(|_| views::KafkaStudio::default()),
        )
        .expect("Failed to open Kafka Studio window");

        cx.activate(true);
    });
}
