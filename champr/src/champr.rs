use std::{borrow::Cow, cell::RefCell, collections::HashMap};

use eframe::egui::{Checkbox, CtxRef, FontDefinitions, FontFamily, TextStyle, TopBottomPanel, Ui};
use serde::{Deserialize, Serialize};

const CHAMPR_APP: &str = "champ-r";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dark_mode: bool,
    pub sources: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dark_mode: false,
            sources: vec![],
        }
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
    pub source_list: RefCell<Vec<SourceItem>>,
    pub selected_sources: RefCell<HashMap<String, bool>>,
    pub config: Config,
}

impl ChampR {
    pub fn new() -> ChampR {
        let source_list: RefCell<Vec<SourceItem>> = RefCell::new(Vec::new());
        for i in 1..50 {
            let name = format!("source {}", i);
            let value = format!("source-{}", i);
            let item = SourceItem::new(name, value);
            source_list.borrow_mut().push(item);
        }

        let config: Config = confy::load(CHAMPR_APP).unwrap_or_default();
        let mut selected_sources = HashMap::new();
        config.sources.iter().for_each(|k| {
            selected_sources.insert(String::from(k), true);
        });
        ChampR {
            source_list,
            selected_sources: RefCell::new(selected_sources),
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
        let list = self.source_list.borrow();
        let mut selected = self.selected_sources.borrow_mut();
        for s in list.iter() {
            ui.group(|ui| {
                let cur = selected.get(&s.value);
                let mut checked = match cur {
                    Some(true) => true,
                    _ => false,
                };

                let c = Checkbox::new(&mut checked, &s.name).text_style(TextStyle::Body);
                let cb = ui.add(c);
                if cb.clicked() {
                    selected.insert(s.value.to_string(), checked);
                    save_config(&self.config, selected.iter());
                }
            });
            ui.add_space(5.);
        }
    }

    pub fn render_top(&mut self, ctx: &CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let btn = ui.button(match self.config.dark_mode {
                false => "🌌",
                true => "💡",
            });

            if btn.clicked() {
                self.config.dark_mode = !self.config.dark_mode;
                save_config(&self.config, self.selected_sources.borrow().iter());
            }

            ui.label("The quick brown fox jumps over the lazy dog");
            ui.label("敏捷的棕色狐狸跨过懒狗");
            ui.separator();
            ui.heading("This is heading #1");
            ui.separator();
        });
    }
}

fn save_config(config: &Config, source_iter: std::collections::hash_map::Iter<String, bool>) {
    let mut sources: Vec<String> = vec![];
    source_iter
        .filter(|&(_k, v)| *v == true)
        .for_each(|(k, _v)| {
            sources.push(String::from(k));
        });

    let r = confy::store(
        CHAMPR_APP,
        Config {
            dark_mode: config.dark_mode,
            sources,
        },
    );
    match r {
        Ok(_) => println!("config saved successfully"),
        Err(e) => println!("save config failed: {}", e),
    }
}
