#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dashboard;
mod data_sources;
mod launcher;
mod theme;

use alvr_sockets::{AudioDevicesList, DashboardRequest};
use dashboard::Dashboard;
use std::{sync::mpsc, thread};

pub enum ServerEvent {
    PingResponseConnected,
    PingResponseDisconnected,
    Event(alvr_events::Event),
    AudioDevicesUpdated(AudioDevicesList),
}

fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();

    let (dashboard_request_sender, dashboard_request_receiver) =
        mpsc::channel::<DashboardRequest>();
    let (server_event_sender, server_event_receiver) = mpsc::channel::<ServerEvent>();

    let data_thread = thread::spawn(|| {
        data_sources::data_interop_thread(dashboard_request_receiver, server_event_sender)
    });
    eframe::run_native(
        "ALVR Dashboard",
        native_options,
        Box::new(|creation_context| {
            Box::new(Dashboard::new(
                creation_context,
                dashboard_request_sender,
                server_event_receiver,
            ))
        }),
    )
    .unwrap();

    data_thread.join().unwrap();
}
