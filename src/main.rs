mod calc;

use std::rc::Rc;

use calc::calculator::Calculator;
use calc::calculator::CalculatorView;

use calc::calculator::TestCalculator;
use calc::calculator::TestCalculator2;
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};

#[derive(Clone, Data, Lens)]
struct CalcView {
    /// The number displayed. Generally a valid float.
    value: String,
    history: String,
}

impl CalculatorView for CalcView {
    fn set_result(self: &mut Self, result: String) {
        self.value = result;
    }

    fn set_history(self: &mut Self, history: String) {
        self.history = history;
    }
}

// impl Data for Calculator<'a, CalcView> {
//     fn same(&self, _: &Self) -> bool {
//         false
//     }
// }

// impl Clone for Calculator<'a, CalcView> {
//     fn clone(&self) -> Self {
//         Self {
//             value: self.value,
//             view: self.view,
//         }
//     }
// }

impl CalcView {
    fn digit(&mut self, digit: u8) {
    }

    fn display(&mut self) {
    }

    fn compute(&mut self) {
    }

    fn op(&mut self, op: char) {
    }
}

fn op_button_label(op: char, label: String) -> impl Widget<CalcView> {
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
        .on_click(move |_ctx, data: &mut CalcView, _env| data.op(op))
}

fn op_button(op: char) -> impl Widget<CalcView> {
    op_button_label(op, op.to_string())
}

fn digit_button(digit: u8) -> impl Widget<CalcView> {
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
        .on_click(move |_ctx, data: &mut CalcView, _env| data.digit(digit))
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
}

fn build_calc() -> impl Widget<CalcView> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(32.0)
        .lens(CalcView::value)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button_label('c', "CE".to_string()),
                op_button('C'),
                op_button('⌫'),
                op_button('÷'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(7),
                digit_button(8),
                digit_button(9),
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
                op_button('+'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button('±'),
                digit_button(0),
                op_button('.'),
                op_button('='),
            ),
            1.0,
        )
}

struct Test {
    /// The number displayed. Generally a valid float.
    value: String,
    history: String,
    caclulator: TestCalculator2<Test>,
}

impl Test {
    fn new() -> Rc<Self> {
        Rc::new_cyclic(|me| {
            // Create the actual struct here.
            Test { 
                value: "0".to_string(),
                history: String::new(),
                caclulator: TestCalculator2::new(me),
            }
        })
    }
}

pub fn main() {
    let window = WindowDesc::new(build_calc())
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );


    let test = Test::new();

    let calc_state: CalcView = CalcView {
        value: "0".to_string(),
        history: String::new(),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(calc_state)
        .expect("launch failed");


}