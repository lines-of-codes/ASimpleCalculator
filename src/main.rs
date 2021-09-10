mod enums;
mod button_wrapper;
use crate::enums::Message;
use crate::enums::Operators;
use crate::button_wrapper::ButtonWrapper;
use fltk::{
    app, 
    prelude::*, 
    window::Window,
    enums::{FrameType, Align},
    frame::Frame,
    group::{Pack, PackType}
};

fn main() {
    println!("Initializing calculator...");
    let app = app::App::default();
    app::set_visible_focus(false);
    app::background(236, 240, 241);

    let win_width = 400;
    let win_height = 450;
    let button_row = 145;

    let mut operation = Operators::None;
    let mut text = "0".to_string();
    let mut old_value = "0".to_string();
    let mut new_value: String;
    let mut expression_text: String = String::new();

    println!("Initializing GUIs...");
    let mut win = Window::new(100, 100, win_width, win_height, "Simple Calculator");

    let mut expression = Frame::new(0, 5, win_width - 10, 25, "").with_align(Align::Right | Align::Inside);
    expression.set_frame(FrameType::FlatBox);
    expression.set_label_size(20);

    let mut output = Frame::new(0, 30, win_width - 10, 110, "").with_align(Align::Right | Align::Inside);
    output.set_frame(FrameType::FlatBox);
    output.set_label_size(26);
    output.set_label("0");

    let vpack = Pack::new(0, button_row, win_width, win_height - 150, "");

    let mut hpack = Pack::new(0, 0, win_width, 60, "");
    let button_ce = ButtonWrapper::new("CE");
    let button_c = ButtonWrapper::new("C");
    let button_del = ButtonWrapper::new("@<-");
    let button_div = ButtonWrapper::new("/");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_width, 60, "");
    let mut button_7 = ButtonWrapper::new("7");
    let mut button_8 = ButtonWrapper::new("8");
    let mut button_9 = ButtonWrapper::new("9");
    let button_mul = ButtonWrapper::new("x");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_width, 60, "");
    let mut button_4 = ButtonWrapper::new("4");
    let mut button_5 = ButtonWrapper::new("5");
    let mut button_6 = ButtonWrapper::new("6");
    let button_sub = ButtonWrapper::new("-");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_width, 60, "");
    let mut button_1 = ButtonWrapper::new("1");
    let mut button_2 = ButtonWrapper::new("2");
    let mut button_3 = ButtonWrapper::new("3");
    let button_add = ButtonWrapper::new("+");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_width, 68, "");
    let mut button_dot = ButtonWrapper::new(".");
    let mut button_0 = ButtonWrapper::new("0");
    let button_equal = ButtonWrapper::new("=");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    vpack.end();

    win.end();
    win.show();

    app::set_focus(&*button_1);
    app::get_system_colors();

    println!("Creating button channel...");
    let button_vector = vec![
        &mut button_1, &mut button_2, &mut button_3, &mut button_4, &mut button_5, &mut button_6,
        &mut button_7, &mut button_8, &mut button_9, &mut button_0,
    ];

    let button_operator_vector = vec![
        button_add, button_sub, button_mul,
        button_div, button_c, button_ce,
        button_del, button_equal
    ];

    let (sender, receiver) = app::channel::<Message>();

    for button in button_vector {
        let label = button.label();
        button.emit(sender, Message::Number(label.parse().unwrap()));
    }

    for mut button in button_operator_vector {
        let label = button.label().clone();
        let op = match label.as_str() {
            "+" => Operators::Add,
            "-" => Operators::Subtract,
            "x" => Operators::Multiply,
            "/" => Operators::Divide,
            "=" => Operators::Equal,
            "CE" => Operators::CE,
            "C" => Operators::C,
            "@<-" => Operators::Delete,
            _ => Operators::None
        };
        button.emit(sender, Message::Operator(op));
    }

    button_dot.emit(sender, Message::Dot);
    println!("Initializing done!");

    while app.wait() {
        if let Some(val) = receiver.recv() {
            match val {
                Message::Number(num) => {
                    if output.label() == "0" {
                        text.clear();
                    }
                    if expression_text.contains("=") {
                        expression_text.clear();
                        expression.set_label(expression_text.as_str());
                    }
                    text.push_str(&num.to_string());
                    output.set_label(text.as_str());
                }
                Message::Dot => {
                    if operation == Operators::Equal {
                        text.clear();
                        operation = Operators::None;
                        output.set_label("0.");
                        text.push_str("0.");
                    }
                    if !text.contains('.') {
                        text.push('.');
                        output.set_label(text.as_str());
                    }
                }
                Message::Operator(op) => match op {
                    Operators::Add | Operators::Subtract | Operators::Multiply | Operators::Divide => {
                        old_value.clear();
                        old_value.push_str(&output.label());
                        expression_text.push_str(old_value.as_str());
                        expression_text.push_str(match op {
                            Operators::Add => " + ",
                            Operators::Subtract => " - ",
                            Operators::Multiply => " * ",
                            Operators::Divide => " / ",
                            _ => ""
                        });
                        operation = op;
                        output.set_label("0");
                        expression.set_label(expression_text.as_str());
                    }
                    Operators::Delete => {
                        let value = output.label();
                        text.pop();
                        if value.len() > 1 {
                            output.set_label(text.as_str());
                        } else {
                            output.set_label("0")
                        }
                    }
                    Operators::CE => {
                        expression_text.clear();
                        text.clear();
                        old_value.clear();
                        text.push('0');
                        output.set_label(text.as_str());
                        expression.set_label(expression_text.as_str());
                    }
                    Operators::C => { 
                        if expression_text.contains('=') {
                            expression_text.clear();
                        }
                        text.clear();
                        text.push('0');
                        output.set_label(text.as_str());
                        expression.set_label(expression_text.as_str());
                    }
                    Operators::Equal => {
                        new_value = output.label();
                        let old: f64 = old_value.parse().unwrap();
                        let new: f64 = new_value.parse().unwrap();
                        let val = match operation {
                            Operators::Add => old + new,
                            Operators::Subtract => old - new,
                            Operators::Multiply => old * new,
                            Operators::Divide => old / new,
                            _ => new,
                        };
                        expression_text.push_str(&(new_value + " ="));
                        let mut final_result_text = val.to_string();
                        // (Check if the user is trying to divide by zero)
                        if operation == Operators::Divide && new == 0.0 {
                            final_result_text = "Cannot divide by zero".to_string();
                        }
                        operation = Operators::None;
                        text = "0".to_string();
                        output.set_label(&final_result_text);
                        expression.set_label(expression_text.as_str());
                    }
                    _ => (),
                }
            }
        }
    }
}
