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

const INDENTATION_STEP: f32 = 20.0;

#[derive(Clone)]
pub struct NestingInfo {
    pub path: Vec<PathSegment>,
    pub indentation_level: usize,
}

pub enum SettingControl {
    Section(section::Control),
    Choice(choice::Control),
    None,
}

impl SettingControl {
    pub fn new(nesting_info: NestingInfo, schema: SchemaNode) -> Self {
        match schema {
            SchemaNode::Section(entries) => {
                Self::Section(section::Control::new(nesting_info, entries))
            }
            SchemaNode::Choice {
                default,
                variants,
                gui,
            } => Self::Choice(choice::Control::new(nesting_info, default, variants, gui)),
            // SchemaNode::Optional { default_set, content } => todo!(),
            // SchemaNode::Switch { default_enabled, content } => todo!(),
            // SchemaNode::Boolean { default } => todo!(),
            // SchemaNode::Integer { default, min, max, step, gui } => todo!(),
            // SchemaNode::Float { default, min, max, step, gui } => todo!(),
            // SchemaNode::Text { default } => todo!(),
            // SchemaNode::Array(_) => todo!(),
            // SchemaNode::Vector { default_element, default } => todo!(),
            // SchemaNode::Dictionary { default_key, default_value, default } => todo!(),
            _ => Self::None,
        }
    }

    // inline: first field child, could be rendered beside the field label
    pub fn ui(
        &self,
        ui: &mut Ui,
        session_fragment: &mut json::Value,
        inline: bool,
    ) -> Option<DashboardRequest> {
        match self {
            SettingControl::Section(control) => control.ui(ui, session_fragment, inline),
            SettingControl::Choice(control) => control.ui(ui, session_fragment, inline),
            SettingControl::None => None,
        }
    }
}
