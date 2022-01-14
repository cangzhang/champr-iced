use iced::{
    button, scrollable, text_input, Button, Checkbox, Column, Container, Length, Row, Sandbox,
    Scrollable, Settings, Text, TextInput,
};

fn main() -> Result<(), iced::Error> {
    let mut settings = Settings::default();
    settings.window.size = (320, 540);
    settings.window.resizable = false;
    SourceList::run(settings)
}

#[derive(Clone)]
struct SourceItem {
    value: String,
    label: String,
}

#[derive(Default)]
struct SourceList {
    variants: Variant,
    input: text_input::State,
    input_value: String,
    btn: button::State,

    items: Vec<SourceItem>,
    selected: Vec<String>,    
}

impl SourceList {
    pub fn new() -> Self {
        let mut items = vec![];
        for i in 1..20 {
            let item = SourceItem {
                label: format!("Source {}", i),
                value: format!("source-{}", i),
            };
            items.push(item);
        }

        Self {
            items,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSource(bool, String),
    OnInput(String),
    OnClick,
}

impl Sandbox for SourceList {
    type Message = Message;

    fn new() -> Self {
        SourceList::new()
    }

    fn title(&self) -> String {
        String::from("ChampR - rust")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleSource(checked, s) => {
                if checked {
                    self.selected.push(s);
                } else {
                    let idx = self.selected.iter().position(|i| i.to_string() == s).unwrap();
                    self.selected.remove(idx);
                }
                println!("{:?}", self.selected);
            }
            Message::OnInput(s) => {
                self.input_value = s;
            }
            Message::OnClick => {
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let search_label = Text::new("Search:");
        let search_input = TextInput::new(
            &mut self.input,
            "type something",
            &self.input_value,
            Message::OnInput,
        )
        .padding(4)
        .width(Length::FillPortion(5));

        let row = Row::new()
            .spacing(10)
            .padding(10)
            .align_items(iced::Align::Center)
            .push(search_label)
            .push(search_input)
            .height(Length::FillPortion(1));

        let mut col = Column::new()
            .spacing(10)
            .push(row)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut scrollable = Scrollable::new(&mut self.variants.scrollable)
            .padding(10)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::FillPortion(5));

        for i in self.items.iter() {
            let label = i.label.to_string();
            let value = i.value.to_string();
            let checked = self.selected.contains(&value);
            let visible = label.contains(&self.input_value);

            if visible {
                let cb = Checkbox::new(checked, label, move |checked| {
                    Message::ToggleSource(checked, value.to_string())
                });
                scrollable = scrollable.push(cb);
            }
        }

        col = col.push(scrollable);
        col = col.push(
            Container::new(
                Button::new(&mut self.btn, Text::new("Click me"))
                    .on_press(Message::OnClick),
            )
            .padding(10)
            .center_x()
            .center_y()
            .height(Length::FillPortion(3)),
        );
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
