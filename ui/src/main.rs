use std::{collections::HashMap};

use iced::{
    scrollable, text_input, Checkbox, Column, Sandbox, Settings, TextInput, 
};

fn main() -> Result<(), iced::Error> {
    SourceList::run(Settings::default())
}

#[derive(Clone)]
struct SourceItem {
    value: String,
    name: String,
}

#[derive(Default)]
struct SourceList {
    scroll: scrollable::State,
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
        Self::default()
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
        let mut col = Column::new().spacing(10).push(TextInput::new(
            &mut self.input,
            "type to search",
            &self.input_value,
            Message::OnInput,
        ));

        for i in 1..10 {
            let label = format!("source-{}", i);
            let val = label.to_string();
            let checked = self.selected.get(&label).unwrap_or(&false);
            let cb = Checkbox::new(*checked, label, move |checked| {
                Message::ToggleSource(checked, String::from(&val))
            });
            col = col.push(cb);
        }

        col.into()
    }
}
