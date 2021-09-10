use fltk::{
    prelude::*,
    button::Button,
    enums::{FrameType, Color, Event, Shortcut, Key}
};
use std::ops::{Deref, DerefMut};

pub struct ButtonWrapper {
    btn: Button
}

impl ButtonWrapper {
    pub fn new(title: &'static str) -> ButtonWrapper {
        let mut button = ButtonWrapper {
            btn: Button::new(0, 0, 100, 0, title),
        };
        button.btn.set_label_size(22);
        button.btn.set_frame(FrameType::FlatBox);
        // (Set the color and shortcut according to the title)
        match title {
            "CE" => {
                button.btn.set_color(Color::from_hex(0xc0392b));
                button.btn.handle(move |button, event| match event {
                    Event::Enter => {
                        button.set_color(Color::from_hex(0xe74c3c));
                        button.redraw();
                        true
                    }
                    Event::Leave => {
                        button.set_color(Color::from_hex(0xc0392b));
                        button.redraw();
                        true
                    }
                    _ => false
                });
                button.btn.set_selection_color(Color::from_hex(0xd50000));
                button.btn.set_shortcut(Shortcut::None | Key::Delete);
            }
            "x" | "/" | "+" | "-" | "=" | "C" | "@<-" => {
                button.btn.set_color(Color::from_hex(0xf1c40f));
                button.btn.handle(move |button, event| match event {
                    Event::Enter => {
                        button.set_color(Color::from_hex(0xf39c12));
                        button.redraw();
                        true
                    }
                    Event::Leave => {
                        button.set_color(Color::from_hex(0xf1c40f));
                        button.redraw();
                        true
                    }
                    _ => false
                });
                button.btn.set_selection_color(Color::from_hex(0xf39c12));
                button.btn.set_label_color(Color::Black);
                let shortcut = if title == "x" {
                    '*'
                } else {
                    title.chars().next().unwrap()
                };
                button.btn.set_shortcut(Shortcut::None | shortcut);
                if shortcut == '@' {
                    button.btn.set_shortcut(Shortcut::None | Key::BackSpace);
                }
                if shortcut == '=' {
                    button.btn.set_shortcut(Shortcut::None | Key::Enter);
                }
            }
            _ => {
                if title == "0" {
                    button.btn.resize(0, 0, 200, 0);
                }
                button.btn.set_label_color(Color::White);
                button.btn.set_color(Color::from_hex(0x2c3e50));
                button.btn.set_selection_color(Color::from_hex(0x34495e));
                button.btn.set_shortcut(Shortcut::None | title.chars().next().unwrap());
                button.btn.handle(move |button, event| match event {
                    Event::Enter => {
                        button.set_color(Color::from_hex(0x34495e));
                        button.redraw();
                        true
                    }
                    Event::Leave => {
                        button.set_color(Color::from_hex(0x2c3e50));
                        button.redraw();
                        true
                    }
                    _ => false
                });
            }
        }
        button
    }
}

impl Deref for ButtonWrapper {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for ButtonWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}