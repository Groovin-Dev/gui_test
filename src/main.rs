use iced::widget::{Button, Column, Text};
use iced::{window, Sandbox};
use iced::{Element, Settings};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (150, 150),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };

    Counter::run(settings)
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(
                Button::new("Increment")
                    .width(150)
                    .on_press(Message::Increment),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new("Decrement")
                    .width(150)
                    .on_press(Message::Decrement),
            )
            .into()
    }
}
