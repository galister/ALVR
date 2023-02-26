use crate::dashboard::DisplayString;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};

// todo: use a custom widget
pub fn button_group_clicked(
    ui: &mut Ui,
    options: &[DisplayString],
    selection: &mut String,
) -> bool {
    let mut clicked = false;
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        for id in options {
            if ui
                .selectable_value(selection, (**id).clone(), &id.display)
                .clicked()
            {
                *selection = (**id).to_owned();
                clicked = true;
            }
        }
    });

    clicked
}
