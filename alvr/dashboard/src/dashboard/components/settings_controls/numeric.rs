// use super::{SettingControl, SettingsContext, SettingsResponse};
// use egui::{emath::Numeric, DragValue, Slider, Ui};
// use serde::{de::DeserializeOwned, Serialize};
// use serde_json as json;
// use settings_schema::NumericGuiType;
// use std::ops::RangeInclusive;

use std::{fmt::Display, str::FromStr};

use super::{reset, NestingInfo};
use alvr_sockets::DashboardRequest;
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use json::Number;
use serde_json as json;
use settings_schema::NumericGuiType;

pub struct Control {
    nesting_info: NestingInfo,
    prev_value: Number,
    prev_string_value: String,
    editing_value: Option<String>,
    default: Number,
    default_string: String,
    gui: NumericGuiType,
}

impl Control {
    pub fn new<T: Display>(
        nesting_info: NestingInfo,
        default: T,
        min: Option<T>,
        max: Option<T>,
        step: Option<T>,
        gui: Option<NumericGuiType>,
    ) -> Self {
        let default_string = format!("\"{default}\"");

        Self {
            nesting_info,
            prev_value: Number::from_str("0").unwrap(),
            prev_string_value: "".into(),
            editing_value: None,
            default: Number::from_str(&default.to_string()).unwrap(),
            default_string,
            gui: gui.unwrap_or(NumericGuiType::TextBox),
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
        let session_number = if let json::Value::Number(number) = session_fragment.clone() {
            number
        } else {
            unreachable!()
        };

        let mut request = None;

        fn get_request(nesting_info: &NestingInfo, number: Number) -> Option<DashboardRequest> {
            Some(DashboardRequest::SetSingleValue {
                path: nesting_info.path.clone(),
                new_value: json::Value::Number(number),
            })
        }

        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            if session_number != self.prev_value {
                self.prev_value = session_number.clone();
                self.prev_string_value = self.prev_value.to_string();
            }

            if let Some(editing_value_mut) = &mut self.editing_value {
                if ui.text_edit_singleline(editing_value_mut).lost_focus() {
                    if let Ok(number) = Number::from_str(editing_value_mut) {
                        request = get_request(&self.nesting_info, number.clone());
                        *session_fragment = json::Value::Number(number);
                    }
                    self.editing_value = None;
                }
            } else if ui
                .text_edit_singleline(&mut self.prev_string_value)
                .gained_focus()
            {
                self.editing_value = Some(self.prev_string_value.clone());
            }

            if reset::reset_button(ui, session_number != self.default, &self.default_string)
                .clicked()
            {
                request = get_request(&self.nesting_info, self.default.clone());
            }
        });

        request
    }
}

// #[derive(Clone)]
// pub enum NumericWidgetType<T> {
//     Slider {
//         range: RangeInclusive<T>,
//         step: f64,
//     },
//     TextBox {
//         range: Option<RangeInclusive<T>>,
//         step: Option<f64>,
//     },
// }

// pub struct NumericWidget<T> {
//     value: T,
//     default: T,
//     numeric_type: NumericWidgetType<T>,
// }

// impl<T: DeserializeOwned + Copy + Numeric> NumericWidget<T> {
//     pub fn new(
//         session_fragment: json::Value,
//         default: T,
//         min: Option<T>,
//         max: Option<T>,
//         step: Option<T>,
//         gui: Option<NumericGuiType>,
//         integer: bool,
//     ) -> Self {
//         let step = if let Some(step) = step {
//             Some(step.to_f64())
//         } else {
//             integer.then(|| 1_f64)
//         };

//         let initial_value = json::from_value::<T>(session_fragment).unwrap();

//         let gui = gui.unwrap_or(NumericGuiType::Slider);

//         let numeric_type = if let (Some(min), Some(max), Some(step)) = (min, max, step) {
//             let range = min..=max;

//             match gui {
//                 NumericGuiType::Slider => NumericWidgetType::Slider { range, step },
//                 NumericGuiType::UpDown => NumericWidgetType::TextBox {
//                     range: Some(range),
//                     step: Some(step),
//                 },
//                 NumericGuiType::TextBox => NumericWidgetType::TextBox {
//                     range: Some(range),
//                     step: None,
//                 },
//             }
//         } else {
//             let range = if let (Some(min), Some(max)) = (min, max) {
//                 Some(min..=max)
//             } else {
//                 None
//             };

//             let step = if matches!(gui, NumericGuiType::TextBox) {
//                 None
//             } else {
//                 step
//             };

//             NumericWidgetType::TextBox { range, step }
//         };

//         Self {
//             value: initial_value,
//             default,
//             numeric_type,
//         }
//     }
// }

// impl<T: Serialize + DeserializeOwned + Numeric + ToString> SettingControl for NumericWidget<T> {
//     fn ui(
//         &mut self,
//         ui: &mut Ui,
//         session_fragment: json::Value,
//         ctx: &SettingsContext,
//     ) -> Option<SettingsResponse> {
//         let response = match self.numeric_type.clone() {
//             NumericWidgetType::Slider { range, step } => {
//                 // todo: handle step
//                 let slider_response =
//                     ui.add(Slider::new(&mut self.value, range).clamp_to_range(true));

//                 if slider_response.drag_released() || slider_response.lost_focus() {
//                     Some(super::into_fragment(self.value))
//                 } else {
//                     // Drag events are not captured for the included textbox, so its value gets overridden. todo: make a PR
//                     if !slider_response.dragged() && !slider_response.has_focus() {
//                         self.value = json::from_value::<T>(session_fragment).unwrap();
//                     }

//                     None
//                 }
//             }
//             NumericWidgetType::TextBox { range, step } => {
//                 // egui supports prefixes/suffixes. todo: add support for suffixes in
//                 // settings-schema
//                 let mut textbox = DragValue::new(&mut self.value);
//                 if let Some(range) = range {
//                     textbox = textbox.clamp_range(range);
//                 }
//                 if let Some(step) = step {
//                     textbox = textbox.speed(step.to_f64());
//                 } else {
//                     textbox = textbox.speed(0);
//                 }

//                 let res = ui.add(textbox);

//                 if res.drag_released() || res.lost_focus() {
//                     Some(super::into_fragment(self.value))
//                 } else {
//                     if !res.dragged() && !res.has_focus() {
//                         // if not currently interacting with the control, overwrite the value from
//                         // the session continuously
//                         self.value = json::from_value::<T>(session_fragment).unwrap();
//                     }
//                     None
//                 }
//             }
//         };

//         super::reset_clicked(
//             ui,
//             &self.value,
//             &self.default,
//             &self.default.to_string(),
//             &ctx.t,
//         )
//         .then(|| super::into_fragment(self.default))
//         .or(response)
//     }
// }
