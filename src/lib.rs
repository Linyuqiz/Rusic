use iced::alignment::Horizontal;
use iced::theme;
use iced::widget::Text;
use iced::Length;
use iced::{
    widget::{column, container, Button},
    Sandbox, Theme,
};
use message::audio::AudioTo;
use message::pages::PageTo;
use message::style::StyleTo;
use message::theme::ThemeTo;
use message::Message;
mod message;

enum Audio {
    Palying,
    Stopping,
    Pausing,
}
enum Page {
    Main,
    Settings,
}
pub struct Application {
    audio: Audio,
    page: Page,
    // custom_style: CustomStyle,
    theme: iced::Theme,
}

// impl container::StyleSheet for CustomStyle {
//     type Style = Theme;
//     fn appearance(&self, style: &Self::Style) -> container::Appearance {
//         container::Appearance {
//             background: Some(iced::Background::Color(match self {
//                 Self::Standard => iced::Color::from_rgb8(0, 200, 100),
//                 Self::Lovely => iced::Color::from_rgb8(255, 100, 90),
//                 Self::Flashy => iced::Color::from_rgb8(255, 0, 0),
//             })),
//             text_color: Some(match style {
//                 Theme::Light => iced::Color::BLACK,
//                 Theme::Dark => iced::Color::WHITE,
//                 _ => iced::Color::default(),
//             }),
//             border: match self {
//                 Self::Standard => Border::default(),
//                 Self::Lovely => Border::with_radius(20),
//                 Self::Flashy => Border::with_radius(-5),
//             },
//             ..Default::default()
//         }
//     }
// }

impl Sandbox for Application {
    type Message = message::Message;
    fn new() -> Self {
        let audio = Audio::Pausing;
        let page = Page::Main;
        // let custom_style = CustomStyle::Standard;
        let theme = iced::Theme::Moonfly;
        Self { audio, page, theme }
    }

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    fn title(&self) -> String {
        String::from("Rusic")
    }
    fn update(&mut self, message: Self::Message) {
        use message::{audio::AudioTo, pages::PageTo, style::StyleTo, theme::ThemeTo, Message};

        match message {
            Message::Page(page_to) => match page_to {
                PageTo::Main => self.page = Page::Main,
                PageTo::Settings => self.page = Page::Settings,
            },
            Message::Style(style_to) => match style_to {
                StyleTo::Standard => {}
                StyleTo::Flashy => {}
                StyleTo::Lovely => {}
            },
            Message::Theme(theme_to) => match theme_to {
                ThemeTo::Dark => self.theme = Theme::Dark,
                ThemeTo::Light => self.theme = Theme::Light,
                ThemeTo::Moonfly => self.theme = Theme::Moonfly,
                ThemeTo::Oxocarbon => self.theme = Theme::Oxocarbon,
            },
            Message::Audio(audio_to) => match audio_to {
                AudioTo::Pause => self.audio = Audio::Pausing,
                AudioTo::Play => self.audio = Audio::Palying,
                AudioTo::Stop => self.audio = Audio::Stopping,
            },
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        match self.page {
            Page::Main => container(column!(
                Button::new(
                    Text::from("ToSet")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Page(PageTo::Settings)),
                // .style(theme::Button::custom(StyleTo::Standard)),
                Button::new(
                    Text::from("Play")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Play))
                .style(theme::Button::custom(StyleTo::Standard)),
                Button::new(
                    Text::from("Pause")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Pause))
                .style(theme::Button::custom(StyleTo::Standard)),
                Button::new(
                    Text::from("Stop")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .on_press(Message::Audio(AudioTo::Stop))
                .style(theme::Button::custom(StyleTo::Standard)),
            ))
            .align_x(Horizontal::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),

            Page::Settings => container(column!(
                Button::new(
                    Text::from("ToMain")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .height(90)
                .width(200)
                .on_press(Message::Page(PageTo::Main))
                .style(theme::Button::custom(StyleTo::Standard)),
                Button::new(
                    Text::from("ToDark")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .height(90)
                .width(200)
                .on_press(Message::Theme(ThemeTo::Dark))
                .style(theme::Button::custom(StyleTo::Standard)),
                Button::new(
                    Text::from("ToLight")
                        .size(19)
                        .horizontal_alignment(Horizontal::Center)
                )
                .height(90)
                .width(200)
                .on_press(Message::Theme(ThemeTo::Light))
                .style(theme::Button::custom(StyleTo::Standard)),
            ))
            .align_x(Horizontal::Left)
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),
        }
    }
}
