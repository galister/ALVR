use crate::ServerEvent;
use alvr_common::once_cell::sync::Lazy;
use alvr_filesystem::{self as afs, Layout};
use alvr_server_data::ServerDataManager;
use alvr_sockets::DashboardRequest;
use std::{sync::mpsc, thread, time::Duration};

// Abstraction layer between local and remote interaction with server IO functions
// struct ServerConnection {

// }

// impl ServerConnection {
//     fn new() {

//     }
// }

pub fn data_interop_thread(
    receiver: mpsc::Receiver<DashboardRequest>,
    sender: mpsc::Sender<ServerEvent>,
) {
    // Note: this is an instance to the server IO manager. To make sure there is one and only ground
    // truth for the data, prefer querying the server if it's alive.

    let session_file_path = afs::filesystem_layout_from_openvr_driver_root_dir(
        &alvr_commands::get_driver_dir().unwrap(),
    )
    .session();

    let mut server_data_manager = Some(ServerDataManager::new(&session_file_path));

    let port = server_data_manager
        .unwrap()
        .settings()
        .connection
        .web_server_port;

    let dashboard_request_uri = format!("http://localhost:{port}/api/dashboard-request");
    let server_events_uri = format!("http://localhost:{port}/api/events");

    thread::spawn(|| {});

    let request_agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_millis(100))
        .build();

    while let Ok(request) = receiver.recv() {
        match request_agent.get(&dashboard_request_uri).send_json(request) {
            Ok(response) => {
                // response.
            }
            Err(ureq::Error::Status(status_code, _)) => {}
            Err(ureq::Error::Transport(_)) => {}
        }
       
    }
}

fn get_gpu_names() -> Vec<