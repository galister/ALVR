mod basic_components;
mod components;

use self::components::{ConnectionsTab, InstallationTab, LogsTab, SettingsTab, SetupWizard};
use crate::{
    dashboard::components::StatisticsTab,
    steamvr_launcher::{self, LAUNCHER},
    theme, ServerEvent,
};
use alvr_common::{prelude::*, RelaxedAtomic};
use alvr_events::{Event, EventSeverity, EventType, LogEvent};
use alvr_session::{ClientConnectionDesc, LogLevel, SessionDesc};
use alvr_sockets::{ClientListAction, DashboardRequest};
use eframe::{
    egui::{
        self, style::Margin, Align, CentralPanel, Context, Frame, Label, Layout, RichText,
        ScrollArea, SidePanel, Stroke, TopBottomPanel, Window,
    },
    epaint::Color32,
};
use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{atomic::AtomicUsize, mpsc, Arc},
    thread,
};

const NOTIFICATION_BAR_HEIGHT: f32 = 30.0;

#[derive(Clone)]
pub struct DisplayString {
    pub id: String,
    pub display: String,
}

impl From<(String, String)> for DisplayString {
    fn from((id, display): (String, String)) -> Self {
        Self { id, display }
    }
}

impl Deref for DisplayString {
    type Target = String;

    fn deref(&self) -> &String {
        &self.id
    }
}

fn get_id() -> usize {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Tab {
    Connections,
    Statistics,
    Settings,
    Installation,
    Logs,
    About,
}

pub struct Dashboard {
    connected_to_server: bool,
    server_restarting: Arc<RelaxedAtomic>,
    selected_tab: Tab,
    tab_labels: BTreeMap<Tab, &'static str>,
    connections_tab: ConnectionsTab,
    statistics_tab: StatisticsTab,
    settings_tab: SettingsTab,
    installation_tab: InstallationTab,
    logs_tab: LogsTab,
    notification: Option<LogEvent>,
    setup_wizard: Option<SetupWizard>,
    session: SessionDesc,
    dashboard_requests_sender: mpsc::Sender<DashboardRequest>,
    server_events_receiver: mpsc::Receiver<ServerEvent>,
}

impl Dashboard {
    pub fn new(
        creation_context: &eframe::CreationContext<'_>,
        dashboard_requests_sender: mpsc::Sender<DashboardRequest>,
        server_events_receiver: mpsc::Receiver<ServerEvent>,
    ) -> Self {
        dashboard_requests_sender
            .send(DashboardRequest::GetSession)
            .unwrap();
        dashboard_requests_sender
            .send(DashboardRequest::GetAudioDevices)
            .unwrap();

        theme::set_theme(&creation_context.egui_ctx);

        Self {
            connected_to_server: false,
            server_restarting: Arc::new(RelaxedAtomic::new(false)),
            selected_tab: Tab::Connections,
            tab_labels: [
                (Tab::Connections, "🔌  Connections"),
                (Tab::Statistics, "📈  Statistics"),
                (Tab::Settings, "⚙  Settings"),
                (Tab::Installation, "💾  Installation"),
                (Tab::Logs, "📝  Logs"),
                (Tab::About, "ℹ  About"),
            ]
            .into_iter()
            .collect(),
            connections_tab: ConnectionsTab::new(),
            statistics_tab: StatisticsTab::new(),
            settings_tab: SettingsTab::new(),
            installation_tab: InstallationTab::new(),
            logs_tab: LogsTab::new(),
            notification: None,
            setup_wizard: None,
            session: SessionDesc::default(),
            dashboard_requests_sender,
            server_events_receiver,
        }
    }
}

impl eframe::App for Dashboard {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        for event in self.server_events_receiver.try_iter() {
            match event {
                ServerEvent::Event(event) => {
                    match event.event_type {
                        EventType::GraphStatistics(graph_statistics) => self
                            .statistics_tab
                            .update_graph_statistics(graph_statistics),
                        EventType::Statistics(statistics) => {
                            self.statistics_tab.update_statistics(statistics)
                        }
                        EventType::Session(session) => {
                            self.session = *session.clone();
                            self.settings_tab.update_session(&session.session_settings);
                        }
                        EventType::ServerRequestsSelfRestart => {
                            if !self.server_restarting.value() {
                                self.server_restarting.set(true);

                                let server_restarting = Arc::clone(&self.server_restarting);
                                thread::spawn(move || {
                                    LAUNCHER.lock().restart_steamvr();

                                    server_restarting.set(false);
                                });
                            }
                        }
                        _ => {
                            self.logs_tab.update_logs(event.clone());
                            // Create a notification based on the notification level in the settings
                            // match self.session.to_settings().extra.notification_level {
                            //     LogLevel::Debug => self.notification = Some(log.to_owned()),
                            //     LogLevel::Info => match log.severity {
                            //         EventSeverity::Info | EventSeverity::Warning | EventSeverity::Error => {
                            //             self.notification = Some(log.to_owned())
                            //         }
                            //         _ => (),
                            //     },
                            //     LogLevel::Warning => match log.severity {
                            //         EventSeverity::Warning | EventSeverity::Error => {
                            //             self.notification = Some(log.to_owned())
                            //         }
                            //         _ => (),
                            //     },
                            //     LogLevel::Error => match log.severity {
                            //         EventSeverity::Error => self.notification = Some(log.to_owned()),
                            //         _ => (),
                            //     },
                            // }
                        }
                    }
                }
                // ServerEvent::DriverResponse(drivers) => {
                //     self.dashboard.new_drivers(drivers);
                // }
                ServerEvent::PingResponseConnected => {
                    self.connected_to_server = true;
                    self.installation_tab.update_drivers();
                }
                ServerEvent::PingResponseDisconnected => {
                    self.connected_to_server = false;
                    self.installation_tab.update_drivers();
                }
                ServerEvent::AudioDevicesUpdated(list) => {
                    self.settings_tab.update_audio_devices(list);
                }
                _ => (),
            }
        }

        if self.server_restarting.value() {
            CentralPanel::default().show(context, |ui| {
                // todo: find a way to center both vertically and horizontally
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading(RichText::new("StreamVR is restarting").size(30.0));
                });
            });

            return;
        }

        let mut requests = vec![];

        match &mut self.setup_wizard {
            Some(setup_wizard) => {
                CentralPanel::default().show(context, |ui| {
                    if let Some(request) = setup_wizard.ui(ui) {
                        requests.push(request);
                    }
                });
            }
            None => {
                // if match &self.notification {
                //     Some(log) => {
                //         let (fg, bg) = match log.severity {
                //             EventSeverity::Debug => (theme::FG, theme::DEBUG),
                //             EventSeverity::Info => (theme::BG, theme::INFO),
                //             EventSeverity::Warning => (theme::BG, theme::WARNING),
                //             EventSeverity::Error => (theme::FG, theme::ERROR),
                //         };
                //         TopBottomPanel::bottom("bottom_panel")
                //             .default_height(NOTIFICATION_BAR_HEIGHT)
                //             .min_height(NOTIFICATION_BAR_HEIGHT)
                //             .frame(
                //                 Frame::default()
                //                     .inner_margin(Margin::same(5.0))
                //                     .fill(bg)
                //                     .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
                //             )
                //             .show(ctx, |ui| {
                //                 ui.horizontal(|ui| {
                //                     ui.add(
                //                         Label::new(RichText::new(&log.content).color(fg))
                //                             .wrap(true),
                //                     );
                //                     ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                //                         ui.button("❌").clicked()
                //                     })
                //                     .inner
                //                 })
                //                 .inner
                //             })
                //             .inner
                //     }
                //     None => {
                //         TopBottomPanel::bottom("bottom_panel")
                //             .default_height(NOTIFICATION_BAR_HEIGHT)
                //             .min_height(NOTIFICATION_BAR_HEIGHT)
                //             .frame(
                //                 Frame::default()
                //                     .inner_margin(Margin::same(5.0))
                //                     .fill(theme::LIGHTER_BG)
                //                     .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
                //             )
                //             .show(ctx, |ui| ui.label("No new notifications"));
                //         false
                //     }
                // } {
                //     self.notification = None;
                // }

                SidePanel::left("side_panel")
                    .resizable(false)
                    .frame(
                        Frame::none()
                            .fill(theme::LIGHTER_BG)
                            .inner_margin(Margin::same(7.0))
                            .stroke(Stroke::new(1.0, theme::SEPARATOR_BG)),
                    )
                    .exact_width(150.0)
                    .show(context, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                            ui.add_space(13.0);
                            ui.heading(RichText::new("ALVR").size(25.0).strong());
                            egui::warn_if_debug_build(ui);
                        });

                        ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                            for (tab, label) in &self.tab_labels {
                                ui.selectable_value(&mut self.selected_tab, *tab, *label);
                            }
                        });

                        ui.with_layout(
                            Layout::bottom_up(Align::Center).with_cross_justify(true),
                            |ui| {
                                ui.add_space(5.0);
                                if self.connected_to_server {
                                    if ui.button("Restart SteamVR").clicked() {
                                        requests.push(DashboardRequest::RestartSteamvr);
                                    }
                                } else if ui.button("Launch SteamVR").clicked() {
                                    thread::spawn(|| LAUNCHER.lock().launch_steamvr());
                                }

                                ui.horizontal(|ui| {
                                    ui.add_space(5.0);
                                    ui.label("Streamer:");
                                    ui.add_space(-10.0);
                                    if self.connected_to_server {
                                        ui.label(RichText::new("Connected").color(Color32::GREEN));
                                    } else {
                                        ui.label(RichText::new("Disconnected").color(Color32::RED));
                                    }
                                })
                            },
                        )
                    });

                CentralPanel::default()
                    .frame(
                        Frame::none()
                            .inner_margin(Margin::same(20.0))
                            .fill(theme::BG),
                    )
                    .show(context, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            ui.heading(
                                RichText::new(*self.tab_labels.get(&self.selected_tab).unwrap())
                                    .size(25.0),
                            );
                            ScrollArea::new([false, true]).show(ui, |ui| match self.selected_tab {
                                Tab::Connections => {
                                    if let Some(request) = self.connections_tab.ui(
                                        ui,
                                        &self.session,
                                        self.connected_to_server,
                                    ) {
                                        requests.push(request);
                                    }
                                }
                                Tab::Statistics => {
                                    if let Some(request) = self.statistics_tab.ui(ui) {
                                        requests.push(request);
                                    }
                                }
                                Tab::Settings => {
                                    requests.extend(self.settings_tab.ui(ui));
                                }
                                Tab::Installation => self.installation_tab.ui(ui),
                                Tab::Logs => {
                                    if let Some(request) = self.logs_tab.ui(ui) {
                                        requests.push(request);
                                    }
                                }
                                Tab::About => components::about_tab_ui(ui),
                            })
                        })
                    });
            }
        };

        for request in requests {
            self.dashboard_requests_sender.send(request).ok();
        }
    }

    fn on_close_event(&mut self) -> bool {
        true
    }
}
