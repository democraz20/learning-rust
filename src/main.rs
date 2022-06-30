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
    theme: style::Theme,
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
                .style(self.theme)
            )
            .push(
                Text::new(self.value.to_string()).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                .on_press(Message::DecrementPressed)
                .padding([10, 50])
                .style(self.theme)
            )
            .push(
                Text::new(self.alert_message.to_string()).size(50),
            )
            // .style(self.theme)
            .align_items(Alignment::Center);
        
        let container1 = container::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .style(self.theme)
            .into();
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


#[allow(unused_must_use)]
mod style {
    #[allow(unused_imports)]
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

    impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Dark => dark::Container.into(),
            }
        }
    }
    
    impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => light::Button.into(),
                Theme::Dark => dark::Button.into(),
            }
        }
    }

    mod light {
        use iced::{button, Color, Vector};

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Color::from_rgb(0.11, 0.42, 0.87).into(),
                    border_radius: 12.0,
                    shadow_offset: Vector::new(1.0, 1.0),
                    text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
        }
    }

    #[allow(unused_must_use)]
    mod dark {
        #[allow(unused_imports)]
        use iced::{
            button, checkbox, container, progress_bar, radio, rule, scrollable,
            slider, text_input, toggler, Color,
        };

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
                    text_color: Color::WHITE.into(),
                    ..container::Style::default()
                }
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: ACTIVE.into(),
                    border_radius: 3.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: HOVERED.into(),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1.0,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }
    }
}