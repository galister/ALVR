use super::{reset, NestingInfo};
use alvr_sockets::DashboardRequest;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use serde_json as json;

pub struct Control {
    nesting_info: NestingInfo,
    temp_value: Option<String>,
    default: String,
    default_string: String,
}

impl Control {
    pub fn new(nesting_info: NestingInfo, default: String) -> Self {
        let default_string = format!("\"{}\"", default);

        Self {
            nesting_info,
            temp_value: None,
            default,
            default_string,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        allow_inline: bool,
    ) -> Option<DashboardRequest> {
        super::grid_flow_inline(ui, allow_inline);

        // todo: can this be written better?
        let text_mut = if let json::Value::String(text) = session_fragment {
            text
        } else {
            unreachable!()
        };

        let mut request = None;

        fn get_request(nesting_info: &NestingInfo, text: &str) -> Option<DashboardRequest> {
            Some(DashboardRequest::SetSingleValue {
                path: nesting_info.path.clone(),
                new_value: json::Value::String(text.to_owned()),
            })
        }

        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            if let Some(temp_value_mut) = &mut self.temp_value {
                if ui.text_edit_singleline(temp_value_mut).lost_focus() {
                    request = get_request(&self.nesting_info, temp_value_mut);
                    *text_mut = temp_value_mut.clone();
                    self.temp_value = None;
                }
            } else if ui.text_edit_singleline(text_mut).gained_focus() {
                self.temp_value = Some(text_mut.clone());
            }

            if reset::reset_button(ui, *text_mut != self.default, &self.default_string).clicked() {
                request = get_request(&self.nesting_info, &self.default);
            }
        });

        request
    }
}
