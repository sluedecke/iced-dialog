use std::sync::Mutex;

use iced::widget::{button, column, container, row, text};
use iced::window;
use iced::{executor, Application, Command, Element, Settings, Theme};
use icons::Icon;

mod icons;

struct Dialog;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FontLoaded,
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
        (
            Self,
            iced::font::load(icons::ICONS_DATA).map(|_| Message::FontLoaded),
        )
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
            Message::FontLoaded => {
                println!("Font loaded");
                Command::none()
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
        container(row![
            Icon::InfoSquareRounded.text(),
            column![
                text("Testmessage"),
                row![
                    // buttons
                    button("Cancel").on_press(Message::CancelPressed),
                    button("OK").on_press(Message::OKPressed),
                ],
            ],
        ])
        .padding(24)
        .into()
    }
}

// TODO.2023-12-12 implement
// fn confirmation(title: &str, message: &str) -> Result<bool, Error> {}
// fn alert(title: &str, message: &str) -> Result<(), Error> {}

fn main() {
    println!("Hello, world!");
    // TODO.2023-12-12 escape shall close window
    println!("{:#?}", Dialog::run(Settings::default()));
    println!("Goodbye world: {RR:#?}");
}
