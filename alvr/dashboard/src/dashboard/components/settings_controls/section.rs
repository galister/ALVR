use super::{NestingInfo, SettingControl, INDENTATION_STEP};
use crate::dashboard::DisplayString;
use alvr_sockets::DashboardRequest;
use eframe::egui::Ui;
use serde_json as json;
use settings_schema::{NamedEntry, SchemaNode};
use std::collections::HashMap;

fn get_display_name(id: &str, strings: &HashMap<String, String>) -> String {
    strings.get("display_name").cloned().unwrap_or_else(|| {
        let mut chars = id.chars();
        chars.next().unwrap().to_uppercase().collect::<String>()
            + chars.as_str().replace('_', " ").as_str()
    })
}

struct Entry {
    id: DisplayString,
    help: Option<String>,
    notice: Option<String>,
    control: SettingControl,
}

pub struct Control {
    nesting_info: NestingInfo,
    entries: Vec<Entry>,
}

impl Control {
    pub fn new(mut nesting_info: NestingInfo, schema_entries: Vec<NamedEntry<SchemaNode>>) -> Self {
        if nesting_info.path.len() > 1 {
            nesting_info.indentation_level += 1;
        }

        let entries = schema_entries
            .into_iter()
            .map(|entry| {
                let id = entry.name;
                let display = get_display_name(&id, &entry.strings);
                let help = entry.strings.get("help").cloned();
                let notice = entry.strings.get("notice").cloned();

                let mut nesting_info = nesting_info.clone();
                nesting_info.path.push(id.clone().into());

                Entry {
                    id: DisplayString { id, display },
                    help,
                    notice,
                    control: SettingControl::new(nesting_info, entry.content),
                }
            })
            .collect();

        Self {
            nesting_info,
            entries,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        allow_inline: bool,
    ) -> Option<DashboardRequest> {
        super::grid_flow_block(ui, allow_inline);

        let session_fragments_mut = session_fragment.as_object_mut().unwrap();

        let entries_count = self.entries.len();

        let mut response = None;
        for (i, entry) in self.entries.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(INDENTATION_STEP * self.nesting_info.indentation_level as f32);
                ui.label(&entry.id.display);
            });
            response = entry
                .control
                .ui(ui, &mut session_fragments_mut[&entry.id.id], true)
                .or(response);

            if i != entries_count - 1 {
                ui.end_row();
            }
        }

        response
    }
}
