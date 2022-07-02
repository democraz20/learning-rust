use iced::{
    button, Alignment, Button, Column, Container, Element, Length, Sandbox,
    Settings, Text,
};

pub fn main() -> iced::Result {
    Exit::run(Settings::default())
}

#[derive(Default)]
struct Exit {
    show_confirm: bool,
    show_settings: bool,
    exit: bool,
    confirm_button: button::State,
    exit_button: button::State,
    settings_button: button::State,
    theme: style::Theme
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Confirm,
    Settings,
    Exit,
}

impl Sandbox for Exit {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Exit - Iced")
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Confirm => {
                self.exit = true;
            }
            Message::Settings => {
                self.show_settings = true
            }
            Message::Exit => {
                self.show_confirm = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = 
        if self.show_confirm {
            Column::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(Text::new("Are you sure you want to exit?"))
                .push(
                    Button::new(
                        &mut self.confirm_button,
                        Text::new("Yes, exit now"),
                    )
                    .padding([10, 20])
                    .on_press(Message::Confirm)
                    .style(self.theme)
                )
        } 
        else if self.show_settings {
            Column::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(Text::new("Settings"))
                .push(
                    Button::new(&mut self.settings_button, Text::new("Back"))
                    .padding([10, 20])
                    .on_press(Message::Exit)
                    .style(self.theme)
                )
        } else {
            Column::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(Text::new("Click the button to exit"))
                .push(
                    Button::new(&mut self.exit_button, Text::new("Exit"))
                        .padding([10, 20])
                        .on_press(Message::Exit)
                        .style(self.theme)
                )
                .push(
                    Button::new(&mut self.settings_button, Text::new("Settings"))
                        .padding([10, 20])
                        .on_press(Message::Settings)
                        .style(self.theme)
                )
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
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