//! 桌面程序入口：启动 GPUI、创建主窗口并挂载根视图。

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui::{AppContext, WindowBackgroundAppearance};
use gpui_component::{Root, Theme};
use kafka_studio::views;

/// 启动本地 Demo 窗口；当前阶段不建立 Kafka 连接。
fn main() {
    eprintln!("[kafka-studio] event=app_start mode=demo");
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::global_mut(cx).colors.background.a = 0.0;
        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            // 窗口提供焦点环境，cx.new 为根视图创建受 GPUI 管理的实体。
            |window, cx| {
                let view = cx.new(|cx| views::KafkaStudio::new(window, cx));
                cx.new(|cx| Root::new(view.into(), window, cx))
            },
        )
        .expect("Failed to open Kafka Studio window");

        eprintln!("[kafka-studio] event=window_opened");
        cx.activate(true);
    });
}
