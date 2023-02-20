// use super::{Section, SettingsContext, SettingsResponse};
use crate::{
    dashboard::{basic_components, DashboardRequest},
    DisplayString,
};
use alvr_session::{SessionDesc, SessionSettings};
use egui::Ui;
use egui_extras::{Column, Table, TableBuilder, TableRow};
use serde_json as json;
use settings_schema::SchemaNode;
use std::{collections::HashMap, sync::Arc};

pub struct SettingsTab {
    // table_contents: HashMap<String, Section>,
    // context: SettingsContext,
}

impl SettingsTab {
    pub fn new(session_settings: &SessionSettings) -> Self {
        // let schema = alvr_session::settings_schema(alvr_session::session_settings_default());
        // let mut session = json::from_value::<HashMap<String, json::Value>>(
        //     json::to_value(session_settings).unwrap(),
        // )
        // .unwrap();

        // if let SchemaNode::Section { entries } = schema {
        //     Self {
        //         selected_tab: entries[0].0.clone(),
        //         tab_labels: entries
        //             .iter()
        //             .map(|(id, _)| LocalizedId {
        //                 id: id.clone(),
        //                 trans: trans.get(id),
        //             })
        //             .collect(),
        //         table_contents: entries
        //             .into_iter()
        //             .map(|(id, data)| {
        //                 if let SchemaNode::Section { entries } = data.unwrap().content {
        //                     (
        //                         id.clone(),
        //                         Section::new(entries, session.remove(&id).unwrap(), &id, trans),
        //                     )
        //                 } else {
        //                     panic!("Invalid schema!")
        //                 }
        //             })
        //             .collect(),
        //         context: SettingsContext {
        //             advanced: false,
        //             view_width: 0_f32,
        //             t,
        //         },
        //     }
        // } else {
        //     panic!("Invalid schema!")
        // }

        Self {}
    }

    pub fn ui(&mut self, ui: &mut Ui, session: &SessionDesc) -> Option<DashboardRequest> {
        TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::remainder())
            .body(|mut body| {
                body.row(10.0, |mut row| {
                    row.col(|ui| {
                        ui.button("hello");
                        // (res.rect, res)
                    });
                })
            });

        // self.context.view_width = ui.available_width();

        // let selected_tab = &mut self.selected_tab;

        // let content = self
        //     .table_contents
        //     .iter_mut()
        //     .find_map(|(id, section)| (**id == *selected_tab).then(|| section))
        //     .unwrap();

        // let mut session_tabs = json::from_value::<HashMap<String, json::Value>>(
        //     json::to_value(&session.session_settings).unwrap(),
        // )
        // .unwrap();

        // let mut advanced = self.context.advanced;

        // let response = basic_components::tabs(
        //     ui,
        //     &self.tab_labels,
        //     selected_tab,
        //     {
        //         let selected_tab = selected_tab.clone();
        //         let context = &self.context;
        //         move |ui| {
        //             content
        //                 .ui_no_indentation(
        //                     ui,
        //                     session_tabs.get(&selected_tab).cloned().unwrap(),
        //                     context,
        //                 )
        //                 .and_then(|res| match res {
        //                     SettingsResponse::SessionFragment(tab_session) => {
        //                         session_tabs.insert(selected_tab, tab_session);

        //                         let mut session = session.clone();
        //                         let session_settings = if let Ok(value) =
        //                             json::from_value(json::to_value(session_tabs).unwrap())
        //                         {
        //                             value
        //                         } else {
        //                             //Some numeric fields are not properly validated
        //                             println!("Invalid value");
        //                             return None;
        //                         };

        //                         session.session_settings = session_settings;

        //                         Some(DashboardResponse::SessionUpdated(Box::new(session)))
        //                     }
        //                     SettingsResponse::PresetInvocation(code) => {
        //                         Some(DashboardResponse::PresetInvocation(code))
        //                     }
        //                 })
        //         }
        //     },
        //     {
        //         |ui| {
        //             if ui.selectable_label(advanced, "Advanced").clicked() {
        //                 advanced = !advanced;
        //             }
        //         }
        //     },
        // );

        // self.context.advanced = advanced;

        None
    }
}
