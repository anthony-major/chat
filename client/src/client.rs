use iced::widget::scrollable::{RelativeOffset, Viewport};
use iced::widget::{column, scrollable, text, text_input, Column, Container};
use iced::{executor, Application, Command, Element, Length, Theme};

use crate::message::Message;

pub struct Client {
    username: String,
    input: String,
    messages: Vec<Message>,
    scroll_id: scrollable::Id,
    scrolled_to_bottom: bool,
}

impl Application for Client {
    type Executor = executor::Default;
    type Flags = ();
    type Message = UiMessage;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let scroll_id = scrollable::Id::unique();
        (
            Self {
                username: String::from("User"),
                input: String::new(),
                messages: Vec::new(),
                scroll_id: scroll_id.clone(),
                scrolled_to_bottom: true,
            },
            scrollable::snap_to(scroll_id, RelativeOffset::END),
        )
    }

    fn title(&self) -> String {
        String::from("Chat")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            UiMessage::Scrolled(viewport) => {
                self.scrolled_to_bottom = viewport.relative_offset().y == 1.0;
            },
            UiMessage::Inputted(input) => {
                self.input = input;
            }
            UiMessage::Submitted => {
                self.messages.push(Message::new(self.username.clone(), self.input.clone()));
                self.input.clear();
                if self.scrolled_to_bottom {
                    return scrollable::snap_to(self.scroll_id.clone(), RelativeOffset::END);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let lines: Vec<Element<Self::Message>> = self
            .messages
            .iter()
            .map(|message| text(message.to_string()).into())
            .collect();
        let scroll_column = Column::with_children(lines);
        let chat_box = scrollable(scroll_column)
            .id(self.scroll_id.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .on_scroll(UiMessage::Scrolled);

        let input_box = text_input("Enter message...", &self.input)
            .on_input(UiMessage::Inputted)
            .on_submit(UiMessage::Submitted);

        let main_column = column![chat_box, input_box];

        Container::new(main_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum UiMessage {
    Scrolled(Viewport),
    Inputted(String),
    Submitted,
}
