use gpui::{App, SharedString, Window};
use gpui_component::input::{InputState, TextInput};

/// 以纯文本展示只读内容；稳定的 ID 让选区和焦点在重新渲染时得以保留。
/// 同一视图内各段文本应使用不同的 ID，父容器需要提供可用宽度。
pub(crate) fn selectable_text(
    id: &'static str,
    value: impl Into<SharedString>,
    window: &mut Window,
    cx: &mut App,
) -> TextInput {
    let value = value.into();
    let state = window.use_keyed_state(id, cx, |window, cx| InputState::new(window, cx));
    // 只在展示内容改变时同步，避免普通刷新重置用户正在拖选的范围。
    if state.read(cx).value() != value {
        state.update(cx, |input, cx| input.set_value(value, window, cx));
    }
    // 此版本的 disabled 禁止修改，但保留文本选择和复制；不解析 Markdown。
    TextInput::new(&state)
        .disabled(true)
        .appearance(false)
        .bordered(false)
        .focus_bordered(false)
}
