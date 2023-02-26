use crate::dashboard::basic_components;

use super::{reset, NestingInfo, SettingControl};
use alvr_sockets::DashboardRequest;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use serde_json as json;
use settings_schema::SchemaNode;

pub struct Control {
    nesting_info: NestingInfo,
    default_enabled: bool,
    default_string: String,
    control: Box<SettingControl>,
}

impl Control {
    pub fn new(
        nesting_info: NestingInfo,
        default_enabled: bool,
        schema_content: SchemaNode,
    ) -> Self {
        let default_string = if default_enabled {
            "ON".into()
        } else {
            "OFF".into()
        };

        let control = {
            let mut nesting_info = nesting_info.clone();
            nesting_info.path.push("content".into());

            SettingControl::new(nesting_info, schema_content)
        };

        Self {
            nesting_info,
            default_enabled,
            default_string,
            control: Box::new(control),
        }
    }

    pub fn ui(
        &self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        inline: bool,
    ) -> Option<DashboardRequest> {
        let session_switch = session_fragment.as_object_mut().unwrap();

        // todo: can this be written better?
        let enabled = if let json::Value::Bool(enabled) = &mut session_switch["enabled"] {
            enabled
        } else {
            unreachable!()
        };

        if !inline {
            ui.add_space(1.0);
        }

        let mut request = None;

        fn get_request(nesting_info: &NestingInfo, enabled: bool) -> Option<DashboardRequest> {
            super::set_single_value(nesting_info, "enabled".into(), json::Value::Bool(enabled))
        }

        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            if basic_components::switch(ui, enabled).clicked() {
                request = get_request(&self.nesting_info, *enabled);
            }

            if reset::reset_button(ui, *enabled != self.default_enabled, &self.default_string)
                .clicked()
            {
                request = get_request(&self.nesting_info, self.default_enabled);
            }
        });

        if *enabled {
            ui.end_row();

            request = self
                .control
                .ui(ui, &mut session_switch["content"], false)
                .or(request);
        }

        request
    }
}
