use super::{NestingInfo, SettingControl, INDENTATION_STEP};
use crate::dashboard::DisplayString;
use alvr_sockets::{DashboardRequest, PathSegment};
use eframe::egui::Ui;
use serde_json as json;
use settings_schema::{NamedEntry, SchemaNode};

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
    pub fn new(nesting_info: NestingInfo, schema_entries: Vec<NamedEntry<SchemaNode>>) -> Self {
        let entries = schema_entries
            .into_iter()
            .map(|entry| {
                let id = entry.name;
                let display = super::get_display_name(&id, &entry.strings);
                let help = entry.strings.get("help").cloned();
                let notice = entry.strings.get("notice").cloned();

                let mut nesting_info = nesting_info.clone();
                nesting_info.path.push(PathSegment::Name(id.clone()));
                nesting_info.indentation_level += 1;

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

    pub fn ui(&self, ui: &mut Ui, session_fragment: &json::Value) -> Option<DashboardRequest> {
        // dbg!(session_fragment);
        let session_fragments = session_fragment.as_object().unwrap();

        let mut response = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if i > 0 || self.nesting_info.indentation_level != 0 {
                ui.end_row();
            }

            ui.horizontal(|ui| {
                ui.add_space(INDENTATION_STEP * self.nesting_info.indentation_level as f32);
                ui.label(&entry.id.display);
            });
            response = entry
                .control
                .ui(ui, &session_fragments[&entry.id.id])
                .or(response);

            // if i < self.entries.len() - 1 {
            //     ui.end_row();
            // }
        }

        response
    }
}
