pub mod champr;

use champr::ChampR;
use eframe::{
    egui::{CentralPanel, CtxRef, Hyperlink, ScrollArea, TextStyle, TopBottomPanel, Vec2, Visuals, Label},
    epi::App,
    run_native, NativeOptions,
};

impl App for ChampR {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.render_top(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_list(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "ChampR"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(5.);
            ui.add(Label::new("ChampR rust version"));
            ui.add(
                Hyperlink::new("https://github.com/cangzhang?from=rust_ver")
                    .text("Made with 💛 by al")
                    .text_style(TextStyle::Monospace),
            );
            ui.add_space(5.);
        })
    });
}

fn main() {
    let app = ChampR::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(360., 640.));
    run_native(Box::new(app), win_option);
}
