use super::{reset, NestingInfo, SettingControl};
use crate::dashboard::{basic_components, DisplayString};
use alvr_sockets::DashboardRequest;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use serde_json as json;
use settings_schema::{ChoiceControlType, NamedEntry, SchemaNode};
use std::collections::HashMap;

fn get_display_name(id: &str, strings: &HashMap<String, String>) -> String {
    strings.get("display_name").cloned().unwrap_or_else(|| {
        let mut chars = id.chars();

        let mut new_chars = vec![chars.next().unwrap()];
        for c in chars {
            let new_c = c.to_ascii_lowercase();

            if new_c != c {
                new_chars.push(' ');
            }

            new_chars.push(new_c);
        }

        new_chars.into_iter().collect::<String>()
    })
}

pub struct Control {
    nesting_info: NestingInfo,
    default_variant: String,
    default_string: String,
    variant_labels: Vec<DisplayString>,
    variant_controls: HashMap<String, SettingControl>,
    gui: ChoiceControlType,
}

impl Control {
    pub fn new(
        nesting_info: NestingInfo,
        default: String,
        schema_variants: Vec<NamedEntry<Option<SchemaNode>>>,
        gui: Option<ChoiceControlType>,
    ) -> Self {
        let variant_labels = schema_variants
            .iter()
            .map(|entry| DisplayString {
                id: entry.name.clone(),
                display: get_display_name(&entry.name, &entry.strings),
            })
            .collect::<Vec<_>>();

        let default_string = format!(
            "\"{}\"",
            variant_labels
                .iter()
                .find(|d| d.id == default)
                .cloned()
                .unwrap()
                .display
        );

        let variant_controls = schema_variants
            .into_iter()
            .map(|entry| {
                let mut nesting_info = nesting_info.clone();
                nesting_info.path.push(entry.name.clone().into());

                let control = if let Some(schema) = entry.content {
                    SettingControl::new(nesting_info, schema)
                } else {
                    SettingControl::None
                };

                (entry.name, control)
            })
            .collect();

        Self {
            nesting_info,
            default_variant: default,
            default_string,
            variant_labels,
            variant_controls,
            gui: gui.unwrap_or(ChoiceControlType::ButtonGroup),
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        allow_inline: bool,
    ) -> Option<DashboardRequest> {
        super::grid_flow_inline(ui, allow_inline);

        let session_variants_mut = session_fragment.as_object_mut().unwrap();

        // todo: can this be written better?
        let variant_mut = if let json::Value::String(variant) = &mut session_variants_mut["variant"]
        {
            variant
        } else {
            unreachable!()
        };

        fn get_request(nesting_info: &NestingInfo, variant: &str) -> Option<DashboardRequest> {
            super::set_single_value(
                nesting_info,
                "variant".into(),
                json::Value::String(variant.to_owned()),
            )
        }

        let mut request = None;
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            if basic_components::button_group_clicked(ui, &self.variant_labels, variant_mut) {
                request = get_request(&self.nesting_info, variant_mut);
            }

            if reset::reset_button(
                ui,
                *variant_mut != self.default_variant,
                &self.default_string,
            )
            .clicked()
            {
                request = get_request(&self.nesting_info, &self.default_variant);
            }
        });

        let control = self.variant_controls.get_mut(&*variant_mut).unwrap();
        if !matches!(control, SettingControl::None) {
            ui.end_row();

            //fixes "cannot borrow `*session_variants` as mutable more than once at a time"
            let variant = variant_mut.clone();
            request = control
                .ui(ui, &mut session_variants_mut[&variant], false)
                .or(request);
        }

        request
    }
}
