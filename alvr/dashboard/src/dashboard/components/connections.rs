use crate::{dashboard::DashboardRequest, steamvr_launcher::LAUNCHER, theme};
use alvr_session::SessionDesc;
use alvr_sockets::ClientListAction;
use eframe::{
    egui::{Frame, Layout, RichText, TextEdit, Ui, Window},
    emath::Align2,
    epaint::Color32,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    thread,
};

struct EditPopupState {
    new_client: bool,
    hostname: String,
    ips: Vec<String>,
}

pub struct ConnectionsTab {
    edit_popup_state: Option<EditPopupState>,
}

impl ConnectionsTab {
    pub fn new() -> Self {
        Self {
            edit_popup_state: None,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
        session: &SessionDesc,
        connected_to_server: bool,
    ) -> Option<DashboardRequest> {
        let mut response = None;

        if !connected_to_server {
            const MESSAGE: &str = "The server is not connected! Clients will not be discovered";
            Frame::group(ui.style())
                .fill(theme::WARNING)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(eframe::emath::Align::Center), |ui| {
                            if ui.button("Launch SteamVR").clicked() {
                                thread::spawn(|| LAUNCHER.lock().launch_steamvr());
                            }
                            ui.with_layout(
                                Layout::left_to_right(eframe::emath::Align::Center),
                                |ui| {
                                    ui.add_space(10.0);
                                    ui.heading(RichText::new(MESSAGE).color(Color32::BLACK));
                                },
                            );
                        });
                    });
                });
        }

        // Get the different types of clients from the session
        let (trusted_clients, untrusted_clients) = session
            .client_connections
            .iter()
            .partition::<Vec<_>, _>(|(_, data)| data.trusted);

        Frame::group(ui.style())
            .fill(theme::SECTION_BG)
            .show(ui, |ui| {
                ui.heading("New clients");
                for (hostname, _) in untrusted_clients {
                    ui.horizontal(|ui| {
                        ui.label(hostname);
                        if ui.button("Trust").clicked() {
                            response = Some(DashboardRequest::UpdateClientList {
                                hostname: hostname.clone(),
                                action: ClientListAction::Trust,
                            });
                        };
                    });
                }
            });
        ui.add_space(10.0);
        Frame::group(ui.style())
            .fill(theme::SECTION_BG)
            .show(ui, |ui| {
                ui.heading("Trusted clients");
                for (hostname, data) in trusted_clients {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{hostname}: {} ({})",
                            data.current_ip.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                            data.display_name
                        ));
                        if ui.button("Edit").clicked() {
                            self.edit_popup_state = Some(EditPopupState {
                                new_client: false,
                                hostname: hostname.to_owned(),
                                ips: data
                                    .manual_ips
                                    .iter()
                                    .map(|addr| addr.to_string())
                                    .collect::<Vec<String>>(),
                            });
                        }
                        if ui.button("Remove").clicked() {
                            response = Some(DashboardRequest::UpdateClientList {
                                hostname: hostname.clone(),
                                action: ClientListAction::RemoveEntry,
                            });
                        }
                    });
                }
            });

        ui.add_space(10.0);

        if ui.button("Add client manually").clicked() {
            self.edit_popup_state = Some(EditPopupState {
                hostname: "XXXX.client.alvr".into(),
                new_client: true,
                ips: Vec::new(),
            });
        }

        if let Some(mut state) = self.edit_popup_state.take() {
            Window::new("Edit connection")
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .resizable(false)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.columns(2, |ui| {
                        ui[0].label("Hostname:");
                        ui[1].add_enabled(
                            state.new_client,
                            TextEdit::singleline(&mut state.hostname),
                        );
                        ui[0].label("IP Addresses:");
                        for address in &mut state.ips {
                            ui[1].text_edit_singleline(address);
                        }
                        if ui[1].button("Add new").clicked() {
                            state.ips.push("192.168.1.2".to_string());
                        }
                    });
                    ui.columns(2, |ui| {
                        if ui[0].button("Ok").clicked() {
                            let manual_ips =
                                state.ips.iter().filter_map(|s| s.parse().ok()).collect();

                            if state.new_client {
                                response = Some(DashboardRequest::UpdateClientList {
                                    hostname: state.hostname.clone(),
                                    action: ClientListAction::AddIfMissing {
                                        trusted: true,
                                        manual_ips,
                                    },
                                });
                            } else {
                                response = Some(DashboardRequest::UpdateClientList {
                                    hostname: state.hostname.clone(),
                                    action: ClientListAction::SetManualIps(manual_ips),
                                });
                            }
                        } else if !ui[1].button("Cancel").clicked() {
                            self.edit_popup_state = Some(state);
                        }
                    })
                });
        }

        response
    }
}
