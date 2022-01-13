use std::collections::HashMap;

use iced::{
    scrollable, text_input, Checkbox, Column, Length, Sandbox, Scrollable, Settings, Text,
    TextInput,
};

fn main() -> Result<(), iced::Error> {
    let mut settings = Settings::default();
    settings.window.size = (320, 540);
    SourceList::run(settings)
}

#[derive(Clone)]
struct SourceItem {
    value: String,
    name: String,
}

#[derive(Default)]
struct SourceList {
    variants: Variant,
    selected: HashMap<String, bool>,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSource(bool, String),
    OnInput(String),
}

impl Sandbox for SourceList {
    type Message = Message;

    fn new() -> Self {
        Self {
            variants: Variant::new(),
            selected: HashMap::new(),
            input: text_input::State::new(),
            input_value: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("ChampR - rust")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleSource(checked, s) => {
                self.selected.insert(s, checked);
                println!("{:?}", self.selected);
            }
            Message::OnInput(s) => {
                self.input_value = s;
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new()
            .spacing(10)
            .push(TextInput::new(
                &mut self.input,
                "type to search",
                &self.input_value,
                Message::OnInput,
            ))
            .width(Length::Fill)
            .height(Length::Fill);

        let mut scrollable = Scrollable::new(&mut self.variants.scrollable)
            .padding(10)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::FillPortion(7));

        for i in 1..20 {
            let label = format!("source-{}", i);
            let val = label.to_string();
            let checked = self.selected.get(&label).unwrap_or(&false);
            let cb = Checkbox::new(*checked, label, move |checked| {
                Message::ToggleSource(checked, String::from(&val))
            });
            scrollable = scrollable.push(cb);
        }

        col = col.push(scrollable);
        col = col.push(Text::new("this is some text").height(Length::FillPortion(3)));
        col.into()
    }
}

#[derive(Default)]
struct Variant {
    scrollable: scrollable::State,
}

impl Variant {
    pub fn new() -> Self {
        Self {
            scrollable: scrollable::State::new(),
        }
    }
}
