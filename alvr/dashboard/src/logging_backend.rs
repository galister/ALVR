use alvr_common::log::LevelFilter;

pub fn init_logging() {
    env_logger::Builder::new()
        .filter(Some("alvr_events"), LevelFilter::Off)
        .filter(Some("naga"), LevelFilter::Off)
        .filter(Some("ureq"), LevelFilter::Off)
        .filter(Some("wgpu_core"), LevelFilter::Off)
        .filter(Some("wgpu_hal"), LevelFilter::Off)
        .filter_level(if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();
}
