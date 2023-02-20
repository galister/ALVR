#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod dashboard;
mod data_sources;
mod launcher;
mod theme;

use alvr_sockets::DashboardRequest;
use dashboard::{ALVRDashboard, Dashboard};
use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

pub enum ServerEvent {
    Event(alvr_events::Event),
    DriverResponse(Vec<String>),
    LostConnection(String),
    Connected,
}

// fn dashboard_requests_thread(receiver: mpsc::Receiver<DashboardRequest>, sender: mpsc::Sender<ServerEvent>) {

// }

// fn server_events_thread() {
//     loop {
//         let mut( socket, response)= tungstenite::connect("ws://localhost:8082")
//     }
// }

fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();

    let (server_event_sender, server_event_receiver) = mpsc::channel::<ServerEvent>();
    let (dashboard_request_sender, dashboard_request_receiver) =
        mpsc::channel::<DashboardRequest>();

    let data_thread = thread::spawn(|| {
        data_sources::data_interop_thread(dashboard_request_receiver, server_event_sender)
    });
    eframe::run_native(
        "ALVR Dashboard",
        native_options,
        Box::new(|creation_context| {
            Box::new(ALVRDashboard::new(
                creation_context,
                dashboard_request_sender,
                server_event_receiver,
            ))
        }),
    )
    .unwrap();

    data_thread.join().unwrap();
}
