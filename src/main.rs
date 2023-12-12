use std::sync::Mutex;

use iced::{
    executor,
    widget::{button, column, row, text},
    window, Application, Command, Element, Settings, Theme,
};

struct Dialog;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CancelPressed,
    OKPressed,
}

#[derive(Debug)]
pub enum DialogResult {
    Cancel,
    OK,
}

// NOTE.2023-12-12 this is not really thread safe, since multiple dialogs from
// different threads will try to modify this variable, though mutexed.
pub static RR: Mutex<Option<DialogResult>> = Mutex::new(None);

impl Application for Dialog {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Ludusavi")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CancelPressed => {
                println!("Cancel pressed");
                *RR.lock().unwrap() = Some(DialogResult::Cancel);
                window::close()
            }
            Message::OKPressed => {
                println!("OK pressed");
                *RR.lock().unwrap() = Some(DialogResult::OK);
                window::close()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        // +------------------------------+
        // |         Ludusavi             |
        // |  ===                         |
        // |   =     Message              |
        // |  ===                         |
        // |               [Cancel] [OK]  |
        // +------------------------------+
        row![
            // icon
            // TODO.2023-12-11 icon
            column![
                text("Testmessage"),
                row![
                    // buttons
                    button("Cancel").on_press(Message::CancelPressed),
                    button("OK").on_press(Message::OKPressed),
                ],
            ],
        ]
        .into()
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:#?}", Dialog::run(Settings::default()));
    println!("Goodbye world: {RR:#?}");
}
