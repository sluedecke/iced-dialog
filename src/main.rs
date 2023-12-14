use std::sync::Mutex;

use gilrs::{Button, Event, Gilrs};
use iced::futures::SinkExt;
use iced::widget::{button, column, container, row, text};
use iced::{executor, subscription, Application, Command, Element, Length, Settings, Theme};
use iced::{window, Subscription};
use icons::Icon;

mod icons;

/// The result of the user interaction
#[derive(Debug, Copy, Clone)]
pub enum DialogResult {
    Undefined,
    Cancel,
    OK,
}

struct Dialog {
    has_gamepad: bool,
    flags: DialogFlags,
}

/// Type of dialog, either confirmation or alert
#[derive(Default, PartialEq)]
enum DialogType {
    #[default]
    Confirmation,
    Alert,
}

/// Configuration flags for the dialog
#[derive(Default)]
struct DialogFlags {
    title: String,
    message: String,
    dialog_type: DialogType,
}

/// Messages to be passed within the dialog
#[derive(Debug, Clone, Copy)]
enum Message {
    FontLoaded,
    CancelPressed,
    OKPressed,
    GamepadButton(Button),
    GamepadConnected,
    GamepadDisconnected,
}

/// Gilrs loop to capture gamepad events as subscriptions
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
pub static DIALOG_RESULT_MUTEX: Mutex<DialogResult> = Mutex::new(DialogResult::Undefined);

impl Application for Dialog {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = DialogFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
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
                flags,
            },
            iced::font::load(icons::ICONS_DATA).map(|_| Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        self.flags.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        println!("Message received: {message:?}");
        match message {
            Message::CancelPressed | Message::GamepadButton(Button::East) => {
                println!("Cancel pressed");
                *DIALOG_RESULT_MUTEX.lock().unwrap() = DialogResult::Cancel;
                window::close()
            }
            Message::OKPressed | Message::GamepadButton(Button::South) => {
                println!("OK pressed");
                *DIALOG_RESULT_MUTEX.lock().unwrap() = DialogResult::OK;
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
        // +--------------------------------+
        // |         Ludusavi               |
        // |  ===                           |
        // |   =     Message                |
        // |  ===                           |
        // |         [(B) Cancel] [(A) OK]  |
        // +--------------------------------+

        let label_cancel = match self.has_gamepad {
            true => row![Icon::XboxB.text_small(), "Cancel"].spacing(4),
            false => row!["Cancel"],
        };

        let label_ok = match self.has_gamepad {
            true => row![Icon::XboxA.text_small(), "OK"].spacing(4),
            false => row!["OK"],
        };

        let mut button_row = row![]
            .spacing(8)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center);
        if self.flags.dialog_type == DialogType::Confirmation {
            button_row = button_row.push(button(label_cancel).on_press(Message::CancelPressed));
        }
        button_row = button_row.push(button(label_ok).on_press(Message::OKPressed));

        let icon = match self.flags.dialog_type {
            DialogType::Confirmation => Icon::InfoSquareRounded.text(),
            DialogType::Alert => Icon::AlertSquareRounded.text(),
        };

        container(
            row![
                icon,
                column![
                    text(self.flags.message.clone())
                        .vertical_alignment(iced::alignment::Vertical::Top)
                        .height(Length::Fill),
                    button_row,
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

pub fn confirmation(title: &str, message: &str) -> Result<DialogResult, iced::Error> {
    // TODO.2023-12-12 escape shall close window
    Dialog::run(Settings {
        window: window::Settings {
            size: (300, 150),
            ..Default::default()
        },
        exit_on_close_request: false,
        flags: DialogFlags {
            title: title.to_string(),
            message: message.to_string(),
            dialog_type: DialogType::Confirmation,
        },
        ..Default::default()
    })?;

    Ok(*DIALOG_RESULT_MUTEX.lock().unwrap())
}

pub fn alert(title: &str, message: &str) -> Result<DialogResult, iced::Error> {
    // TODO.2023-12-12 escape shall close window
    Dialog::run(Settings {
        window: window::Settings {
            size: (300, 150),
            ..Default::default()
        },
        exit_on_close_request: false,
        flags: DialogFlags {
            title: title.to_string(),
            message: message.to_string(),
            dialog_type: DialogType::Alert,
        },
        ..Default::default()
    })?;

    Ok(*DIALOG_RESULT_MUTEX.lock().unwrap())
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", alert("Ludusavi", "Something went wrong!"));
    println!("{:?}", confirmation("Ludusavi", "Confirmation requested"));
    println!("Goodbye world!");
}
