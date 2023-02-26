use std::collections::HashMap;

use super::{NestingInfo, SettingControl};
use crate::dashboard::{basic_components, DisplayString};
use alvr_sockets::DashboardRequest;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use serde_json as json;
use settings_schema::{ChoiceControlType, NamedEntry, SchemaNode};

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
            .collect();

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
            variant_labels,
            variant_controls,
            gui: gui.unwrap_or(ChoiceControlType::ButtonGroup),
        }
    }

    pub fn ui(
        &self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        inline: bool,
    ) -> Option<DashboardRequest> {
        let session_variants = session_fragment.as_object_mut().unwrap();

        // todo: can this be written better?
        let variant = if let json::Value::String(string) = &mut session_variants["variant"] {
            string
        } else {
            unreachable!()
        };

        if !inline {
            ui.add_space(1.0);
        }

        let mut request = None;
        let clicked = ui
            .with_layout(Layout::left_to_right(Align::Center), |ui| {
                basic_components::button_group_clicked(ui, &self.variant_labels, variant)
            })
            .inner;

        if clicked {
            let mut path = self.nesting_info.path.clone();
            path.push("variant".into());

            let new_value = json::Value::String(variant.clone());

            request = Some(DashboardRequest::SetSingleValue { path, new_value });
        }

        let control = &self.variant_controls[&*variant];
        if !matches!(control, SettingControl::None) {
            ui.end_row();

            //fixes "cannot borrow `*session_variants` as mutable more than once at a time"
            let variant = variant.clone();
            request = control
                .ui(ui, &mut session_variants[&variant], false)
                .or(request);
        }

        request
    }
}

// impl SettingControl for ChoiceControl {
//     fn ui(
//         &mut self,
//         ui: &mut Ui,
//         session_fragment: json::Value,
//         ctx: &SettingsContext,
//     ) -> Option<SettingsResponse> {
//         let mut session_variants =
//             json::from_value::<HashMap<String, json::Value>>(session_fragment).unwrap();
//         let mut variant =
//             json::from_value(session_variants.get("variant").cloned().unwrap()).unwrap();

//         let response =
//             basic_components::button_group_clicked(ui, &self.variant_labels, &mut variant).then(
//                 || {
//                     session_variants
//                         .insert("variant".to_owned(), json::to_value(&variant).unwrap());

//                     super::into_fragment(&session_variants)
//                 },
//             );

//         let response = super::reset_clicked(
//             ui,
//             &variant,
//             &self.default,
//             &format!("\"{}\"", self.default.display),
//             &ctx.t,
//         )
//         .then(|| {
//             session_variants.insert(
//                 "variant".to_owned(),
//                 json::to_value(&*self.default).unwrap(),
//             );
//             super::into_fragment(&session_variants)
//         })
//         .or(response);

//         let (control, advanced) = self.controls.get_mut(&variant).unwrap();
//         let session_variant = session_variants
//             .get(&variant)
//             .cloned()
//             .unwrap_or(json::Value::Null);

//         (!*advanced || ctx.advanced)
//             .then(|| {
//                 super::map_fragment(control.ui(ui, session_variant, ctx), |session_variant| {
//                     session_variants.insert(variant, session_variant);

//                     session_variants
//                 })
//             })
//             .flatten()
//             .or(response)
//     }
// }

// pub struct ChoiceContainer {
//     containers: HashMap<String, (Box<dyn SettingContainer>, bool)>,
// }

// impl ChoiceContainer {
//     pub fn new(
//         variants_schema: Vec<(String, Option<EntryData>)>,
//         session_fragment: json::Value,
//         trans_path: &str,
//         trans: &TranslationBundle,
//     ) -> Self {
//         let mut session_variants =
//             json::from_value::<HashMap<String, json::Value>>(session_fragment).unwrap();

//         Self {
//             containers: variants_schema
//                 .into_iter()
//                 .map(|(id, data)| {
//                     if let Some(data) = data {
//                         (
//                             id.clone(),
//                             (
//                                 super::create_setting_container(
//                                     data.content,
//                                     session_variants.remove(&id).unwrap(),
//                                     &format!("{}-{}", trans_path, id),
//                                     trans,
//                                 ),
//                                 data.advanced,
//                             ),
//                         )
//                     } else {
//                         (id, (Box::new(EmptyContainer) as _, false))
//                     }
//                 })
//                 .collect(),
//         }
//     }
// }

// impl SettingContainer for ChoiceContainer {
//     fn ui(
//         &mut self,
//         ui: &mut Ui,
//         session_fragment: json::Value,
//         context: &SettingsContext,
//     ) -> Option<SettingsResponse> {
//         let mut session_variants =
//             json::from_value::<HashMap<String, json::Value>>(session_fragment).unwrap();
//         let variant = json::from_value(session_variants.get("variant").cloned().unwrap()).unwrap();

//         let (control, advanced) = self.containers.get_mut(&variant).unwrap();
//         let session_variant = session_variants
//             .get(&variant)
//             .cloned()
//             .unwrap_or(json::Value::Null);

//         (!*advanced || context.advanced)
//             .then(|| {
//                 super::map_fragment(
//                     control.ui(ui, session_variant, context),
//                     |session_variant| {
//                         session_variants.insert(variant, session_variant);

//                         session_variants
//                     },
//                 )
//             })
//             .flatten()
//     }
// }
