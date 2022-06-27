use iced::{
    button, Alignment, Button, Column, Row, Element, Sandbox, Settings, Text,
    container, Length
};
// use screenshot::get_screenshot;

#[derive(Default)]
struct Counter{
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("COUNTER test")
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .push(
                Button::new(&mut self.increment_button, Text::new("+"))
                .on_press(Message::IncrementPressed)
                .padding([10, 50])
                
            )
            .push(
                Text::new(self.value.to_string()).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                .on_press(Message::DecrementPressed)
                .padding([10, 50])
            )
            .align_items(Alignment::Center);
        container::Container::new(content).width(Length::Fill).center_x().into()
    }   
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}
// #[allow(unused_must_use)]
// fn main(){

// }
