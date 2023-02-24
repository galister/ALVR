pub mod array;
pub mod boolean;
pub mod choice;
pub mod dictionary;
pub mod help;
pub mod notice;
pub mod numeric;
pub mod optional;
pub mod presets;
pub mod reset;
pub mod section;
pub mod switch;
pub mod text;
pub mod vector;

use alvr_sockets::{DashboardRequest, PathSegment};
use eframe::egui::Ui;
// use serde::Serialize;
use serde_json as json;
use settings_schema::SchemaNode;
use std::collections::HashMap;

const INDENTATION_STEP: f32 = 20.0;

#[derive(Clone)]
pub struct NestingInfo {
    pub path: Vec<PathSegment>,
    pub indentation_level: usize,
}

fn get_display_name(id: &str, strings: &HashMap<String, String>) -> String {
    strings.get("display_name").cloned().unwrap_or_else(|| {
        let mut chars = id.chars();
        chars.next().unwrap().to_uppercase().collect::<String>()
            + chars.as_str().replace('_', " ").as_str()
    })
}

pub enum SettingControl {
    Section(section::Control),
    Todo,
}

impl SettingControl {
    pub fn new(nesting_info: NestingInfo, schema: SchemaNode) -> Self {
        match schema {
            SchemaNode::Section(entries) => {
                Self::Section(section::Control::new(nesting_info, entries))
            }
            // SchemaNode::Choice { default, variants, gui } => todo!(),
            // SchemaNode::Optional { default_set, content } => todo!(),
            // SchemaNode::Switch { default_enabled, content } => todo!(),
            // SchemaNode::Boolean { default } => todo!(),
            // SchemaNode::Integer { default, min, max, step, gui } => todo!(),
            // SchemaNode::Float { default, min, max, step, gui } => todo!(),
            // SchemaNode::Text { default } => todo!(),
            // SchemaNode::Array(_) => todo!(),
            // SchemaNode::Vector { default_element, default } => todo!(),
            // SchemaNode::Dictionary { default_key, default_value, default } => todo!(),
            _ => Self::Todo,
        }
    }

    pub fn ui(&self, ui: &mut Ui, session_fragment: &json::Value) -> Option<DashboardRequest> {
        match self {
            SettingControl::Section(control) => control.ui(ui, session_fragment),
            SettingControl::Todo => None,
        }
    }
}
