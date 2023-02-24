use alvr_sockets::DashboardRequest;
use eframe::egui::Ui;

pub struct Presets;

impl Presets {
    pub fn ui(ui: &mut Ui) -> Option<DashboardRequest> {
        let request = None;
        ui.columns(2, |ui| {
            ui[0].button("hello");
            ui[1].button("world");
        });

        request
    }
}
