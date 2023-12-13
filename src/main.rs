use std::sync::Mutex;

use gilrs::{Event, Gilrs};
use iced::futures::SinkExt;
use iced::widget::{button, column, container, row, text};
use iced::{executor, subscription, Application, Command, Element, Settings, Theme};
use iced::{window, Subscription};
use icons::Icon;

mod icons;

struct Dialog;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FontLoaded,
    CancelPressed,
    OKPressed,
    PrintALine,
}

#[derive(Debug)]
pub enum DialogResult {
    Cancel,
    OK,
}

fn listen_to_gilrs() -> Subscription<Message> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        0,
        |mut output| async move {
            let mut gilrs = Gilrs::new().unwrap();
            loop {
                // Examine new events
                let mut active_gamepad = None;
                let mut message = None;
                while let Some(Event { id, event, time }) = gilrs.next_event() {
                    println!("{:?} New event from {}: {:?}", time, id, event);
                    active_gamepad = Some(id);
                }

                // You can also use cached gamepad state
                if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
                    if gamepad.is_pressed(gilrs::Button::South) {
                        println!("Button South is pressed (XBox - A, PS - X)");
                        message = Some(Message::OKPressed);
                    }
                }
                if let Some(m) = message {
                    let _ = output.send(m).await;
                }
            }
        },
    )
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
        // Iterate over all connected gamepads
        let gilrs = Gilrs::new().unwrap();
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }

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
            Message::PrintALine => {
                println!("PrintALine");
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

    fn subscription(&self) -> Subscription<Self::Message> {
        listen_to_gilrs()
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
