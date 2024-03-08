mod calc;

use std::cell::RefCell;
use std::rc::Rc;

use calc::calculator::Calculator;

use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

#[derive(Clone, Data, Lens)]
struct AppData {
    history: String,
    value: String,
    caculator: Rc<RefCell<Calculator>>,
}


impl AppData {
    fn digit(&mut self, digit: u8) {
    }

    fn display(&mut self) {
    }

    fn compute(&mut self) {
    }

    fn op(&mut self, op: char) {
        let mut caculator = self.caculator.borrow_mut();
        let state = caculator.push_input(op);
        match state.0 {
            Some(history) => self.history = history,
            None => {}
        }
        match state.1 {
            Some(value) => self.value = value,
            None => {}
            
        }
    }
}

fn op_button_label(op: char, label: String) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(label)
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut AppData, _env| data.op(op))
}

fn op_button(op: char) -> impl Widget<AppData> {
    op_button_label(op, op.to_string())
}

fn digit_button(digit: u8) -> impl Widget<AppData> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(format!("{digit}"))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut AppData, _env| data.digit(digit))
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
    w5: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w5, 1.0)
}

fn build_calc() -> impl Widget<AppData> {
    let lb_history = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(14.0)
        .lens(AppData::history)
        .padding(5.0);
    let lb_result = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(14.0)
        .lens(AppData::value)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(lb_history)
        .with_flex_spacer(0.2)
        .with_child(lb_result)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button_label('c', "CE".to_string()),
                op_button('C'),                
                op_button_label('s', "MS".to_string()),
                op_button_label('s', "MR".to_string()),
                op_button('⌫'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button_label('c', "sin".to_string()),
                op_button_label('c', "cos".to_string()),
                op_button_label('c', "tan".to_string()),
                op_button_label('c', "⅟x".to_string()),
                op_button('÷'), //
            ),
            1.0,
        )
        .with_spacer(1.0)        
        .with_flex_child(
            flex_row(
                digit_button(7),
                digit_button(8),
                digit_button(9),
                op_button_label('i', "x²".to_string()),
                op_button('×'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(4),
                digit_button(5),
                digit_button(6),
                op_button_label('i', "√".to_string()),
                op_button('−'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(1),
                digit_button(2),
                digit_button(3),
                op_button_label('i', "ln".to_string()),
                op_button('+'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(0),
                op_button('.'),
                op_button('e'),
                op_button_label('i', "π".to_string()),
                op_button('='),
            ),
            1.0,
        )
}

pub fn main() {
    let window = WindowDesc::new(build_calc())
        .window_size((403., 400.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );

    let app_data: AppData = AppData {
        history: String::new(),
        value: "0".to_string(),
        caculator: Rc::new(RefCell::new(Calculator::new()))
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(app_data)
        .expect("launch failed");
}