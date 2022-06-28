
use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Settings, Text,
    container, Length,
};
// use screenshot::get_screenshot;

#[derive(Default)]
struct Counter{
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    alert_message: String,
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
        // self.alert_message = String::from("test");
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
            .push(
                Text::new(self.alert_message.to_string()).size(50),
            )
            .align_items(Alignment::Center);
            // .style();
        
        let container1 = container::Container::new(content).width(Length::Fill).height(Length::Fill).center_y().center_x().into();
        // let container1: container = container1.container.Container.Style.background(Color:: BLACK);
        // let container1 = container::Container::new(content).width(Length::Fill).center_x().into();
        // container1.Style.background(Color::BLACK)
        container1
    }   
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.alert_message = String::from("Increased Value.");
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.alert_message = String::from("Decreased Value.");
            }
        }
    }
}
mod style {
    use iced::{
        button, checkbox, container, progress_bar, radio, rule, scrollable,
        slider, text_input, toggler,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Light,
        Dark,
    }

    impl Theme {
        pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Dark
        }
    }
}