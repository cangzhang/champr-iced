use std::{borrow::Cow, cell::RefCell, collections::HashMap};

use eframe::egui::{FontDefinitions, FontFamily, Ui, Checkbox, TextStyle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dark_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}

pub struct SourceItem {
    pub name: String,
    pub value: String,
}

impl SourceItem {
    pub fn new(name: String, value: String) -> SourceItem {
        SourceItem { name, value }
    }
}
pub struct ChampR {
    pub sources: RefCell<Vec<SourceItem>>,
    pub selected_sources: RefCell<HashMap<String, bool>>,
    pub config: Config,
}

impl ChampR {
    pub fn new() -> ChampR {
        let sources: RefCell<Vec<SourceItem>> = RefCell::new(Vec::new());
        for i in 1..50 {
            let name = format!("source {}", i);
            let value = format!("source-{}", i);
            let item = SourceItem::new(name, value);
            sources.borrow_mut().push(item);
        }

        let config: Config = confy::load("champ-r").unwrap_or_default();
        ChampR {
            sources,
            selected_sources: RefCell::new(HashMap::new()),
            config,
        }
    }

    pub fn configure_fonts(&self, ctx: &eframe::egui::CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "SansR".to_string(),
            Cow::Borrowed(include_bytes!("../../SourceHanSans.otf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "SansR".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_list(&self, ui: &mut Ui) {
        let sources = self.sources.borrow();
        for s in sources.iter() {
            ui.group(|ui| {
                let mut sources = self.selected_sources.borrow_mut();
                let cur = sources.get(&s.value);
                let mut checked = match cur {
                    Some(true) => true,
                    _ => false,
                };

                let c = Checkbox::new(&mut checked, &s.name).text_style(TextStyle::Body);
                let cb = ui.add(c);
                if cb.clicked() {
                    // `checked` is mutated after `click`
                    sources.insert(String::from(&s.value), checked);
                }
            });
            ui.add_space(5.);
        }
    }

    pub fn save_config(&self) {
        let r = confy::store(
            "champ-r",
            Config {
                dark_mode: self.config.dark_mode,
            },
        );
        match r {
            Ok(_) => println!("config saved successfully"),
            Err(e) => println!("save config failed: {}", e),
        }
    }

    pub fn render_top(&mut self, ui: &mut Ui) {
        let btn = ui.button(if self.config.dark_mode {
            "🌚"
        } else {
            "🌝"
        });
        if btn.clicked() {
            self.config.dark_mode = !self.config.dark_mode;
            self.save_config();
        }

        ui.label("The quick brown fox jumps over the lazy dog");
        ui.label("敏捷的棕色狐狸跨过懒狗");
        ui.separator();
        ui.heading("This is heading #1");
        ui.separator();
    }
}
