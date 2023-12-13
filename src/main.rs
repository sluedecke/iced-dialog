use std::sync::Mutex;

use gilrs::{Button, Event, Gilrs};
use iced::futures::SinkExt;
use iced::widget::{button, column, container, row, text};
use iced::{
    executor, subscription, Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced::{window, Subscription};
use icons::Icon;

mod icons;

struct Dialog {
    has_gamepad: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FontLoaded,
    CancelPressed,
    OKPressed,
    GamepadButton(Button),
    GamepadConnected,
    GamepadDisconnected,
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
                while let Some(Event {
                    id: _,
                    event,
                    time: _,
                }) = gilrs.next_event()
                {
                    // println!("{:?} New event from {}: {:?}", time, id, event);
                    let _ = match event {
                        gilrs::EventType::Connected => output.send(Message::GamepadConnected).await,
                        gilrs::EventType::Disconnected => {
                            output.send(Message::GamepadDisconnected).await
                        }
                        gilrs::EventType::ButtonReleased(button, _) => {
                            output.send(Message::GamepadButton(button)).await
                        }
                        _ => Ok(()),
                    };
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
        (
            Self {
                // NOTE.2023-12-13 to self: this is a way to find first connected gamepad or none
                has_gamepad: Gilrs::new()
                    .unwrap()
                    .gamepads()
                    .find_map(|(_, gamepad)| {
                        if gamepad.is_connected() {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false),
            },
            iced::font::load(icons::ICONS_DATA).map(|_| Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Ludusavi")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CancelPressed | Message::GamepadButton(Button::East) => {
                println!("Cancel pressed");
                *RR.lock().unwrap() = Some(DialogResult::Cancel);
                window::close()
            }
            Message::OKPressed | Message::GamepadButton(Button::South) => {
                println!("OK pressed");
                *RR.lock().unwrap() = Some(DialogResult::OK);
                window::close()
            }
            Message::FontLoaded => {
                println!("Font loaded");
                Command::none()
            }
            Message::GamepadButton(button) => {
                println!("Other GamepadButton: {:#?}", button);
                Command::none()
            }
            Message::GamepadConnected => {
                self.has_gamepad = true;
                Command::none()
            }
            Message::GamepadDisconnected => {
                self.has_gamepad = false;
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
        // |       [(B) Cancel] [(A) OK]  |
        // +------------------------------+

        let label_cancel = match self.has_gamepad {
            true => row![Icon::XboxB.text_small(), "Cancel"].spacing(4),
            false => row!["Cancel"],
        };
        let label_ok = match self.has_gamepad {
            true => row![Icon::XboxA.text_small(), "OK"].spacing(4),
            false => row!["OK"],
        };
        container(
            row![
                Icon::InfoSquareRounded.text(),
                column![
                    text("Testmessage")
                        .vertical_alignment(iced::alignment::Vertical::Top)
                        .height(Length::Fill),
                    row![
                        // normal buttons
                        button(label_cancel).on_press(Message::CancelPressed),
                        button(label_ok).on_press(Message::OKPressed),
                    ]
                    .spacing(8)
                    .width(Length::Fill)
                    .align_items(iced::Alignment::Center)
                ]
                .spacing(8),
            ]
            .spacing(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Start),
        )
        .padding(24)
        .width(Length::Fill)
        .height(Length::Fill)
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
    println!(
        "{:#?}",
        Dialog::run(Settings {
            window: window::Settings {
                size: (300, 150),
                ..Default::default()
            },
            exit_on_close_request: true,
            ..Default::default()
        })
    );
    println!("Goodbye world: {RR:#?}");
}
