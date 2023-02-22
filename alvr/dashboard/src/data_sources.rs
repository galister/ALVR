use crate::ServerEvent;
use alvr_common::{parking_lot::Mutex, prelude::*, RelaxedAtomic, StrResult};
use alvr_events::{Event, EventType};
use alvr_server_data::ServerDataManager;
use alvr_sockets::{DashboardRequest, ServerResponse};
use std::{
    env,
    sync::{mpsc, Arc},
    thread,
    time::{Duration, Instant},
};

enum DataSource {
    Local(ServerDataManager),
    Remote, // Note: the remote (server) is probably living as a separate process in the same PC
}

pub fn data_interop_thread(
    receiver: mpsc::Receiver<DashboardRequest>,
    sender: mpsc::Sender<ServerEvent>,
) {
    let session_file_path =
        alvr_filesystem::filesystem_layout_from_launcher_exe(&env::current_exe().unwrap())
            .session();

    let server_data_manager = ServerDataManager::new(&session_file_path);

    let port = server_data_manager.settings().connection.web_server_port;

    let data_source = Arc::new(Mutex::new(DataSource::Local(server_data_manager)));

    let running = Arc::new(RelaxedAtomic::new(true));

    let events_thread = thread::spawn({
        let sender = sender.clone();
        move || -> StrResult {
            loop {
                let mut ws =
                    match tungstenite::connect(format!("http://localhost:{port}/api/events")) {
                        Ok((ws, _)) => ws,
                        Err(_) => {
                            // note: ping anyway, and check if the channel is still alive,
                            // otherwise exit the thread
                            sender
                                .send(ServerEvent::PingResponseDisconnected)
                                .map_err(err!())?;

                            thread::sleep(Duration::from_secs(1));

                            continue;
                        }
                    };

                loop {
                    match ws.read_message() {
                        Ok(tungstenite::Message::Binary(data)) => {
                            if let Ok(event) = bincode::deserialize(&data) {
                                sender.send(ServerEvent::Event(event)).map_err(err!())?;
                            }
                        }
                        _ => {
                            sender
                                .send(ServerEvent::PingResponseDisconnected)
                                .map_err(err!())?;

                            error!("fdsf");

                            break;
                        }
                    }
                }
            }
        }
    });

    let ping_thread = thread::spawn({
        let sender = sender.clone();
        let data_source = Arc::clone(&data_source);
        let running = Arc::clone(&running);
        move || -> StrResult {
            let dashboard_request_uri = format!("http://localhost:{port}/api/dashboard-request");
            let request_agent = ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(1))
                .build();

            let mut connected = false;

            const PING_INTERVAL: Duration = Duration::from_secs(1);
            let mut deadline = Instant::now();

            loop {
                let response = request_agent
                    .get(&dashboard_request_uri)
                    .send_json(&DashboardRequest::Ping);

                if response.is_ok() {
                    if !connected {
                        *data_source.lock() = DataSource::Remote;
                        sender
                            .send(ServerEvent::PingResponseConnected)
                            .map_err(err!())?;

                        connected = true;
                    }
                } else if connected {
                    *data_source.lock() =
                        DataSource::Local(ServerDataManager::new(&session_file_path));
                    sender
                        .send(ServerEvent::PingResponseDisconnected)
                        .map_err(err!())?;

                    connected = false;
                }

                deadline += PING_INTERVAL;
                while Instant::now() < deadline {
                    if !running.value() {
                        return Ok(());
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    });

    let dashboard_request_uri = format!("http://localhost:{port}/api/dashboard-request");
    let request_agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(1))
        .build();

    while let Ok(request) = receiver.recv() {
        match request_agent
            .get(&dashboard_request_uri)
            .send_json(&request)
        {
            Ok(response) => {
                if let Ok(response) = response.into_json::<ServerResponse>() {
                    match response {
                        ServerResponse::AudioOutputDevices(devices) => {
                            if sender
                                .send(ServerEvent::AudioOutputDevices(devices))
                                .is_err()
                            {
                                break;
                            }
                        }
                        ServerResponse::AudioInputDevices(devices) => {
                            if sender
                                .send(ServerEvent::AudioInputDevices(devices))
                                .is_err()
                            {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(_) => {
                if let DataSource::Local(data_manager) = &mut *data_source.lock() {
                    fn send_session(
                        sender: &mpsc::Sender<ServerEvent>,
                        data_manager: &mut ServerDataManager,
                    ) {
                        sender
                            .send(ServerEvent::Event(Event {
                                timestamp: "".into(),
                                event_type: EventType::Session(Box::new(
                                    data_manager.session().clone(),
                                )),
                            }))
                            .ok();
                    }

                    match request {
                        DashboardRequest::GetSession => {
                            send_session(&sender, data_manager);
                        }
                        DashboardRequest::UpdateSession(session) => {
                            *data_manager.session_mut() = *session;

                            send_session(&sender, data_manager);
                        }
                        DashboardRequest::ExecuteScript(code) => {
                            if let Err(e) = data_manager.execute_script(&code) {
                                error!("Error executing script: {e}");
                            }

                            send_session(&sender, data_manager);
                        }
                        DashboardRequest::UpdateClientList { hostname, action } => {
                            data_manager.update_client_list(hostname, action);

                            send_session(&sender, data_manager);
                        }
                        DashboardRequest::GetAudioOutputDevices => {
                            if let Ok(list) = data_manager.get_audio_devices_list() {
                                sender
                                    .send(ServerEvent::AudioOutputDevices(list.output))
                                    .ok();
                            }
                        }
                        DashboardRequest::GetAudioInputDevices => {
                            if let Ok(list) = data_manager.get_audio_devices_list() {
                                sender
                                    .send(ServerEvent::AudioOutputDevices(list.input))
                                    .ok();
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    running.set(false);

    events_thread.join().ok();
    ping_thread.join().ok();
}
