//! 只读客户端的 Demo 外壳：连接选择、目录折叠与键盘导航。

use gpui::{Div, Entity, FocusHandle, KeyDownEvent, Window, div, prelude::*, px, rgb, rgba};
use gpui_component::input::{InputState, TextInput};

use crate::models::Connection;

/// 根视图保存交互状态，子布局根据这些状态生成界面。
#[derive(Debug)]
pub struct KafkaStudio {
    /// 选中的示例配置索引；Some 不代表已经连接 Kafka。
    selected_connection: Option<usize>,
    /// 目录可见性独立于选择状态，折叠不清除选择。
    expanded: bool,
    /// 跨渲染保留焦点身份，避免每次构建界面重新申请焦点。
    focus_handle: FocusHandle,
    /// 当前会话的连接配置，供侧栏和工作区共同读取。
    conections: Vec<Connection>,
    /// 选中连接的名称草稿，选择时重新加载，输入修改暂不写回连接配置。
    name_input: Entity<InputState>,
    /// 最近一次名称保存的校验提示。
    name_error: Option<&'static str>,
}

impl KafkaStudio {
    /// 创建初始状态并取得窗口焦点，使根视图可以接收键盘事件。
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        let name_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Enter a connection name"));
        eprintln!("[kafka-studio] event=view_initialized selected=None expanded=true");
        Self {
            selected_connection: None,
            expanded: true,
            focus_handle,
            conections: vec![
                Connection {
                    name: String::from("Local Demo"),
                    bootstrap_servers: String::from("localhost:9092"),
                },
                Connection {
                    name: String::from("Test Demo"),
                    bootstrap_servers: String::from("test.example:9092"),
                },
            ],
            name_input,
            name_error: None,
        }
    }

    /// 处理 Esc 清除、上下循环选择、左右折叠展开；隐藏的连接不参与上下导航。
    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 输入框获得焦点时，方向键交给文本编辑，根视图不同时执行导航。
        if !self.focus_handle.is_focused(window) {
            return;
        }
        // 只记录导航状态的实际变化，不记录无关按键或输入文字。
        let previous_state = (self.selected_connection, self.expanded);
        match event.keystroke.key.as_str() {
            "escape" => {
                self.select_connection(None, window, cx);
            }
            "down" if self.expanded => {
                let next_index = match self.selected_connection {
                    None => 0,
                    Some(index) => {
                        if index == self.conections.len() - 1 {
                            0
                        } else {
                            index + 1
                        }
                    }
                };
                self.select_connection(Some(next_index), window, cx);
            }
            "up" if self.expanded => {
                let next_index = match self.selected_connection {
                    None => 0,
                    Some(index) => {
                        // usize 不能表示负数，先判断零边界再做减法。
                        if index != 0 {
                            index - 1
                        } else {
                            self.conections.len() - 1
                        }
                    }
                };
                self.select_connection(Some(next_index), window, cx);
            }
            "left" => {
                self.expanded = false;
                cx.notify();
            }
            "right" => {
                self.expanded = true;
                cx.notify();
            }
            _ => {}
        }
        if previous_state != (self.selected_connection, self.expanded) {
            eprintln!(
                "[kafka-studio] event=keyboard_navigation key={} selected={:?} expanded={}",
                event.keystroke.key, self.selected_connection, self.expanded
            );
        }
    }
    /// 更新选择并加载名称草稿；重新选择会覆盖输入框中尚未保存的内容。
    fn select_connection(
        &mut self,
        selected: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.name_error = None;
        self.selected_connection = selected;
        let name = match selected {
            Some(index) => self.conections[index].name.clone(),
            None => String::new(),
        };
        self.name_input.update(cx, |input, cx| {
            input.set_value(name, window, cx);
        });
        cx.notify();
    }
    /// 将名称草稿保存到当前连接的内存配置，拒绝空白名称。
    fn save_connection_name(&mut self, cx: &mut Context<Self>) {
        let Some(index) = self.selected_connection else {
            self.name_error = Some("Select a connection first");
            cx.notify();
            return;
        };
        let value = self.name_input.read(cx).value(); // read(cx) 借用输入状态；我们只读取
        let value = value.trim();
        if value.is_empty() {
            self.name_error = Some("Connection name cannot be empty");
            cx.notify();
            return;
        }
        self.conections[index].name = value.to_owned();
        self.name_error = None;
        eprintln!("[kafka-studio] event=connection_name_saved index={index}");
        cx.notify();
    }
}

impl Render for KafkaStudio {
    /// 根据当前状态构建布局；渲染可能频繁发生，因此这里不输出交互日志。
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_connection = self
            .selected_connection
            .map(|index| &self.conections[index]);
        div()
            .track_focus(&self.focus_handle)
            // 传入方法而非调用方法，由 listener 在事件到来时取得可变实体。
            .on_key_down(cx.listener(Self::handle_key_down))
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
                    .child(connection_list(
                        cx,
                        self.selected_connection,
                        self.expanded,
                        &self.conections,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(connection_workspace(selected_connection))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .p_6()
                                    .child(TextInput::new(&self.name_input))
                                    .child(
                                        div()
                                            .id("save-name")
                                            .p_2()
                                            .cursor_pointer()
                                            .child("Save name")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.save_connection_name(cx);
                                            })),
                                    )
                                    .child(self.name_error.unwrap_or("")),
                            ),
                    ),
            )
    }
}

/// 构建静态顶栏，不持有交互状态。
pub(super) fn top_menu() -> Div {
    div()
        .flex()
        .items_center()
        .px_4()
        .h(px(48.0))
        .text_lg()
        .child("Rust Kafka Studio")
}

/// 根据选择与展开状态构建侧栏，通过 listener 将鼠标操作写回根视图。
pub(super) fn connection_list(
    cx: &mut Context<KafkaStudio>,
    selected_connection: Option<usize>,
    expanded: bool,
    connections: &[Connection],
) -> Div {
    let mut list = div()
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
                .id("folder")
                .p_2()
                .cursor_pointer()
                .child(if expanded { "▼ Demo" } else { "▶ Demo" })
                .on_click(cx.listener(|this, _event, window, cx| {
                    this.focus_handle.focus(window);
                    this.expanded = !this.expanded;
                    eprintln!(
                        "[kafka-studio] event=folder_clicked expanded={}",
                        this.expanded
                    );
                    cx.notify();
                })),
        );
    if expanded {
        let mut connection_rows = div().flex().flex_col().pl(px(16.0));
        // iter 借用每条 Connection，enumerate 提供用于选择状态的索引。
        for (index, conn) in connections.iter().enumerate() {
            // child 消费容器并返回更新后的容器，必须接回返回值以继续添加行。
            connection_rows = connection_rows.child(
                div()
                    .id(("connection", index))
                    .bg(if selected_connection == Some(index) {
                        rgba(0x334155e6)
                    } else {
                        rgba(0x18181be6)
                    })
                    .p_2()
                    .cursor_pointer()
                    .child(conn.name.clone())
                    // 点击发生在循环结束之后，move 为回调保存这一行的索引副本。
                    .on_click(cx.listener(move |this, _event, window, cx| {
                        this.focus_handle.focus(window);
                        this.select_connection(Some(index), window, cx);
                        eprintln!(
                            "[kafka-studio] event=connection_selected source=mouse index={index} connected=false"
                        );
                    })),
            );
        }
        list = list.child(connection_rows);
    }
    // 清除入口位于折叠区域之外，目录隐藏时仍然可用。
    list = list.child(
        div()
            .id("clear-selection")
            .p_2()
            .cursor_pointer()
            .child("Clear selection")
            .on_click(cx.listener(|this, _event, window, cx| {
                this.focus_handle.focus(window);
                this.select_connection(None, window, cx);
                eprintln!("[kafka-studio] event=selection_cleared source=mouse");
            })),
    );
    list
}

/// 显示选中配置的名称或空状态，始终明确当前尚未连接。
pub(super) fn connection_workspace(conection: Option<&Connection>) -> Div {
    let content = match conection {
        None => div().child("Select a connection"),
        Some(conection) => div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().text_lg().child(conection.name.clone()))
            .child(format!(
                "Bootstrap servers: {}",
                conection.bootstrap_servers
            ))
            .child("Status: not connected"),
    };
    div().flex().flex_1().p_6().child(content)
}
