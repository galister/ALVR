use std::collections::VecDeque;

use crate::theme::log_colors;
use alvr_events::{Event, EventSeverity, EventType};
use eframe::{
    egui::{Grid, Layout, ScrollArea, Ui},
    emath::Align,
    epaint::Color32,
};

struct Entry {
    color: Color32,
    timestamp: String,
    ty: String,
    message: String,
}

pub struct LogsTab {
    show_raw_events: bool,
    entries: VecDeque<Entry>,
    log_limit: usize,
}

impl LogsTab {
    pub fn new() -> Self {
        Self {
            show_raw_events: true,
            entries: VecDeque::new(),
            log_limit: 1000,
        }
    }

    pub fn set_show_raw_events(&mut self, show: bool) {
        self.show_raw_events = show;
    }

    pub fn push_event(&mut self, event: Event) {
        match event.event_type {
            EventType::Log(log_event) => {
                let color;
                let ty;
                match log_event.severity {
                    EventSeverity::Error => {
                        color = log_colors::ERROR_FG;
                        ty = "ERROR";
                    }
                    EventSeverity::Warning => {
                        color = log_colors::WARNING_FG;
                        ty = "WARN";
                    }
                    EventSeverity::Info => {
                        color = log_colors::INFO_FG;
                        ty = "INFO";
                    }
                    EventSeverity::Debug => {
                        color = log_colors::DEBUG_FG;
                        ty = "DEBUG";
                    }
                };

                self.entries.push_back(Entry {
                    color,
                    timestamp: event.timestamp,
                    ty: ty.into(),
                    message: log_event.content,
                });
            }
            event_type => {
                if self.show_raw_events {
                    self.entries.push_back(Entry {
                        color: log_colors::EVENT_FG,
                        timestamp: event.timestamp,
                        ty: "EVENT".into(),
                        message: format!("{event_type:?}"),
                    });
                }
            }
        }

        if self.entries.len() > self.log_limit {
            self.entries.pop_front();
        }
    }

    pub fn ui(&self, ui: &mut Ui) {
        ScrollArea::both().show(ui, |ui| {
            Grid::new(0).num_columns(3).striped(true).show(ui, |ui| {
                for entry in &self.entries {
                    ui.colored_label(entry.color, &entry.timestamp);
                    ui.colored_label(entry.color, &entry.ty);
                    ui.colored_label(entry.color, &entry.message);

                    ui.end_row();
                }
            });
        });
    }
}
