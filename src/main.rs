use iced::{
    executor,
    widget::{button, column, row, text},
    Application, Command, Element, Settings, Theme,
};

struct Dialog;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CancelPressed,
    OKPressed,
}

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
                Command::none()
            }
            Message::OKPressed => {
                println!("OK pressed");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        // +------------------------------+
        // |       Ludusavi               |
        // |   *                          |
        // |   I   Message                |
        // |                              |
        // |               [Cancel] [OK]  |
        // +------------------------------+
        row![
            // icon
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
}
