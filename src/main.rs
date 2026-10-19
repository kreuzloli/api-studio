use api_studio::views;
use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui::{AppContext, WindowBackgroundAppearance};

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(700.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |_, cx| cx.new(|_| views::ApiStudio),
        )
        .expect("Failed to open API Studio window");

        cx.activate(true);
    });
}
