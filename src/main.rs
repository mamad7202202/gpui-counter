use gpui::{
    div, prelude::*, px, rgb, rgba, size, App, Application, Bounds, Context, FontWeight,
    SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions,
};

struct Counter {
    count: i64,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.count {
            0 => SharedString::from("Ready"),
            n if n < 0 => SharedString::from("Below zero!"),
            _ => SharedString::from("Counting up"),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format!("{}", self.count)),
            )
            .child(div().text_sm().text_color(rgb(0xa6adc8)).child(label))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        button(
                            "decrement",
                            "\u{2212}",
                            rgb(0xf38ba8),
                            rgb(0x45253a),
                        )
                        .on_click(cx.listener(|this, _event, _window, _cx| {
                            this.count -= 1;
                        })),
                    )
                    .child(
                        button("increment", "+", rgb(0xa6e3a1), rgb(0x27361f))
                            .on_click(cx.listener(|this, _event, _window, _cx| {
                                this.count += 1;
                            })),
                    ),
            )
    }
}

fn button(
    id: &'static str,
    glyph: &'static str,
    fg: gpui::Rgba,
    bg: gpui::Rgba,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .px_5()
        .py_2()
        .rounded_md()
        .text_xl()
        .text_color(fg)
        .bg(bg)
        .border_1()
        .border_color(rgba(0x89b4fa40))
        .cursor_pointer()
        .hover(|style| style.opacity(0.8))
        .child(glyph)
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(480.), px(320.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI Counter".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Counter { count: 0 }),
        )
        .unwrap();

        cx.activate(true);
    });
}
