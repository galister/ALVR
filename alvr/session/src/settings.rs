use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{DictionaryDefault, SettingsSchema, Switch, SwitchDefault};

include!(concat!(env!("OUT_DIR"), "/openvr_property_keys.rs"));

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum FrameSize {
    Scale(#[schema(gui(slider(min = 0.25, max = 2.0, step = 0.01)))] f32),
    Absolute {
        #[schema(gui(slider(min = 32, max = 0x2000, step = 32)))]
        width: u32,
        #[schema(gui(slider(min = 32, max = 0x2000, step = 32)))]
        height: u32,
    },
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum EncoderQualityPreset {
    Quality = 0,
    Balanced = 1,
    Speed = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum NvencTuningPreset {
    HighQuality = 1,
    LowLatency = 2,
    UltraLowLatency = 3,
    Lossless = 4,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum NvencMultiPass {
    Disabled = 0,
    #[schema(strings(display_name = "1/4 resolution"))]
    QuarterResolution = 1,
    FullResolution = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum NvencAdaptiveQuantizationMode {
    Disabled = 0,
    Spatial = 1,
    Temporal = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum RateControlMode {
    #[schema(strings(display_name = "CBR"))]
    Cbr = 0,
    #[schema(strings(display_name = "VBR"))]
    Vbr = 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum EntropyCoding {
    #[schema(strings(display_name = "CAVLC"))]
    Cavlc = 1,
    #[schema(strings(display_name = "CABAC"))]
    Cabac = 0,
}

/// Except for preset, the value of these fields is not applied if == -1 (flag)
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct NvencOverrides {
    pub tuning_preset: NvencTuningPreset,
    #[schema(strings(
        help = "Reduce compression artifacts at the cost of small performance penalty"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub multi_pass: NvencMultiPass,
    #[schema(strings(
        help = r#"Spatial: Helps reduce color banding, but high-complexity scenes might look worse.
Temporal: Helps improve overall encoding quality, very small trade-off in speed."#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adaptive_quantization_mode: NvencAdaptiveQuantizationMode,
    #[schema(flag = "steamvr-restart")]
    pub low_delay_key_frame_scale: i64,
    #[schema(flag = "steamvr-restart")]
    pub refresh_rate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_intra_refresh: bool,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_period: i64,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_count: i64,
    #[schema(flag = "steamvr-restart")]
    pub max_num_ref_frames: i64,
    #[schema(flag = "steamvr-restart")]
    pub gop_length: i64,
    #[schema(flag = "steamvr-restart")]
    pub p_frame_strategy: i64,
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_buffer_size: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_initial_delay: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_max_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_average_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_weighted_prediction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AmfControls {
    #[schema(flag = "steamvr-restart")]
    pub enable_vbaq: bool,
    #[schema(flag = "steamvr-restart")]
    pub use_preproc: bool,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_sigma: u32,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_tor: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug)]
pub enum MediacodecDataType {
    Float(f32),
    Int32(i32),
    Int64(i64),
    String(String),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]

pub struct AdvancedCodecOptions {
    #[schema(flag = "streamvr-restart")]
    pub encoder_quality_preset: EncoderQualityPreset,

    #[schema(flag = "streamvr-restart")]
    pub nvenc_overrides: NvencOverrides,

    #[schema(flag = "streamvr-restart")]
    pub amf_controls: AmfControls,

    pub mediacodec_extra_options: Vec<(String, MediacodecDataType)>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum BitrateMode {
    #[schema(strings(display_name = "Constant"))]
    ConstantMbps(#[schema(gui(slider(min = 1, max = 1000, logarithmic)), suffix = "Mbps")] u64),
    Adaptive {
        #[schema(strings(
            help = "Percentage of network bandwidth to allocate for video transmission"
        ))]
        #[schema(gui(slider(min = 0.5, max = 2.0, step = 0.05)))]
        saturation_multiplier: f32,

        #[schema(strings(display_name = "Maximum bitrate"))]
        #[schema(gui(slider(min = 1, max = 1000)), suffix = "Mbps")]
        max_bitrate_mbps: Switch<u64>,

        #[schema(strings(display_name = "Minimum bitrate"))]
        #[schema(gui(slider(min = 1, max = 1000)), suffix = "Mbps")]
        min_bitrate_mbps: Switch<u64>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct BitrateConfig {
    pub mode: BitrateMode,

    #[schema(gui(slider(min = 0.01, max = 2.0, step = 0.01)))]
    pub framerate_reset_threshold_multiplier: f32,

    #[schema(strings(display_name = "Maximum network latency"))]
    #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
    pub max_network_latency_ms: Switch<u64>,

    #[schema(strings(
        display_name = "Maximum decoder latency",
        help = "When the decoder latency goes above this threshold, the bitrate will be reduced"
    ))]
    #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
    pub max_decoder_latency_ms: u64,

    #[schema(strings(
        display_name = "Decoder latency overstep",
        help = "Number of consecutive frames above the threshold to trigger a bitrate reduction"
    ))]
    #[schema(gui(slider(min = 1, max = 100)), suffix = " frames")]
    pub decoder_latency_overstep_frames: u64,

    #[schema(strings(
        help = "Controls how much the bitrate is reduced when the decoder latency goes above the threshold"
    ))]
    #[schema(gui(slider(min = 0.5, max = 1.0)))]
    pub decoder_latency_overstep_multiplier: f32,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Copy, Clone)]
pub enum OculusFovetionLevel {
    None,
    Low,
    Medium,
    High,
    HighTop,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct FoveatedRenderingDesc {
    #[schema(strings(display_name = "Center region width"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_x: f32,

    #[schema(strings(display_name = "Center region height"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_y: f32,

    #[schema(strings(display_name = "Center shift X"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_x: f32,

    #[schema(strings(display_name = "Center shift Y"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_y: f32,

    #[schema(strings(display_name = "Horizontal edge ratio"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_x: f32,

    #[schema(strings(display_name = "Vertical edge ratio"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_y: f32,
}

#[repr(C)]
#[derive(SettingsSchema, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct ColorCorrectionDesc {
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub brightness: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub contrast: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub saturation: f32,

    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub gamma: f32,

    #[schema(gui(slider(min = -1.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub sharpening: f32,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone)]
#[schema(gui = "button_group")]
pub enum CodecType {
    #[schema(strings(display_name = "h264"))]
    H264,
    #[schema(strings(display_name = "HEVC"))]
    Hevc,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct VideoDesc {
    #[schema(strings(help = "You probably don't want to change this"))]
    #[schema(flag = "steamvr-restart")]
    pub adapter_index: u32,

    #[schema(strings(
        display_name = "Transcoding resolution",
        help = "Resolution used for encoding and decoding"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub render_resolution: FrameSize,

    #[schema(strings(
        display_name = "Emulated headset resolution",
        help = "This is the resolution that SteamVR will use as a baseline for the game rendering"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub recommended_target_resolution: FrameSize,

    #[schema(strings(display_name = "Preferred FPS"))]
    #[schema(gui(slider(min = 60.0, max = 120.0)), suffix = "Hz")]
    #[schema(flag = "steamvr-restart")]
    pub preferred_fps: f32,

    #[schema(
        strings(
            display_name = "Maximum buffering",
            help = "Incresing this value will help reduce stutter but it will increase latency"
        ),
        gui(slider(min = 1.0, max = 10.0, step = 0.1, logarithmic)),
        suffix = " frames"
    )]
    pub max_buffering_frames: f32,

    #[schema(gui(slider(min = 0.50, max = 0.99, step = 0.01)))]
    pub buffering_history_weight: f32,

    #[schema(strings(
        help = "HEVC may provide better visual fidelity at the cost of increased encoder latency"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub codec: CodecType,

    #[schema(strings(help = r#"CBR: Constant BitRate mode. This is recommended.
VBR: Variable BitRate mode. Not commended because it may throw off the adaptive bitrate algorithm. This is only supported on Windows and only with AMD/Nvidia GPUs"#))]
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: RateControlMode,

    #[schema(strings(
        help = r#"In CBR mode, this makes sure the bitrate does not fall below the assigned value. This is mostly useful for debugging."#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub filler_data: bool,

    #[schema(strings(help = r#"CAVLC algorithm is recommended.
CABAC produces better compression but it's significantly slower and may lead to runaway latency"#))]
    #[schema(flag = "steamvr-restart")]
    pub entropy_coding: EntropyCoding,

    #[schema(strings(
        display_name = "Reduce color banding",
        help = "Sets the encoder to use 10 bits per channel instead of 8. Does not work on Linux with Nvidia"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub use_10bit_encoder: bool,

    #[schema(strings(
        display_name = "Force software encoding",
        help = "Forces the encoder to use CPU instead of GPU"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_sw_encoding: bool,

    #[schema(strings(display_name = "Software encoder thread count"))]
    #[schema(flag = "steamvr-restart")]
    pub sw_thread_count: u32,

    pub bitrate: BitrateConfig,

    #[schema(flag = "steamvr-restart")]
    pub advanced_codec_options: AdvancedCodecOptions,

    #[schema(flag = "steamvr-restart")]
    pub seconds_from_vsync_to_photons: f32,

    #[schema(flag = "steamvr-restart")]
    pub foveated_rendering: Switch<FoveatedRenderingDesc>,

    pub oculus_foveation_level: OculusFovetionLevel,

    pub dynamic_oculus_foveation: bool,

    #[schema(flag = "steamvr-restart")]
    pub color_correction: Switch<ColorCorrectionDesc>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum AudioDeviceId {
    Default,
    Name(String),
    Index(u64),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioBufferingConfig {
    #[schema(strings(display_name = "Average buffering"))]
    #[schema(gui(slider(min = 0, max = 200)), suffix = "ms")]
    pub average_buffering_ms: u64,

    #[schema(strings(display_name = "Batch size"))]
    #[schema(gui(slider(min = 1, max = 20)), suffix = "ms")]
    pub batch_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct GameAudioDesc {
    #[schema(strings(display_name = "Device ID"))]
    pub device_id: AudioDeviceId,
    pub mute_when_streaming: bool,
    pub buffering_config: AudioBufferingConfig,
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct MicrophoneDesc {
    #[schema(strings(display_name = "Input device ID"))]
    pub input_device_id: AudioDeviceId,
    #[cfg(not(target_os = "linux"))]
    #[schema(strings(display_name = "Output device ID"))]
    pub output_device_id: AudioDeviceId,
    pub buffering_config: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[schema(gui = "button_group")]
pub enum LinuxAudioBackend {
    #[schema(strings(display_name = "ALSA"))]
    Alsa,

    Jack,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioSection {
    #[schema(strings(help = "ALSA is recommended for most PulseAudio or PipeWire-based setups"))]
    pub linux_backend: LinuxAudioBackend,

    pub game_audio: Switch<GameAudioDesc>,

    pub microphone: Switch<MicrophoneDesc>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum OpenvrPropValue {
    Bool(bool),
    Float(f32),
    Int32(i32),
    Uint64(u64),
    Vector3([f32; 3]),
    Double(f64),
    String(String),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ControllersTriggerOverrideDesc {
    #[schema(gui(slider(min = 0.01, max = 1.0, step = 0.01)))]
    pub trigger_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ControllersGripOverrideDesc {
    #[schema(gui(slider(min = 0.01, max = 1.0, step = 0.01)))]
    pub grip_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HapticsConfig {
    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.1)))]
    pub intensity_multiplier: f32,

    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub amplitude_curve: f32,

    #[schema(strings(display_name = "Minimum duration"))]
    #[schema(gui(slider(min = 0.0, max = 0.1, step = 0.001)), suffix = "s")]
    pub min_duration_s: f32,

    #[schema(gui(slider(min = 1.0, max = 5.0, step = 0.1)))]
    pub low_duration_amplitude_multiplier: f32,

    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub low_duration_range_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ControllersDesc {
    #[schema(flag = "steamvr-restart")]
    pub mode_idx: i32,

    #[schema(flag = "steamvr-restart")]
    pub tracking_system_name: String,

    #[schema(flag = "steamvr-restart")]
    pub manufacturer_name: String,

    #[schema(flag = "steamvr-restart")]
    pub model_number: String,

    #[schema(flag = "steamvr-restart")]
    pub render_model_name_left: String,

    #[schema(flag = "steamvr-restart")]
    pub render_model_name_right: String,

    #[schema(flag = "steamvr-restart")]
    pub serial_number: String,

    #[schema(flag = "steamvr-restart")]
    pub ctrl_type_left: String,

    #[schema(flag = "steamvr-restart")]
    pub ctrl_type_right: String,

    #[schema(flag = "steamvr-restart")]
    pub registered_device_type: String,

    #[schema(flag = "steamvr-restart")]
    pub input_profile_path: String,

    #[schema(strings(
        display_name = "Pose time offset",
        help = "This controls how smooth the controllers should track"
    ))]
    #[schema(gui(slider(min = -1000, max = 1000, logarithmic)), suffix = "ms")]
    pub pose_time_offset_ms: i64,

    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "m/s")]
    pub linear_velocity_cutoff: f32,

    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 100.0, step = 1.0)), suffix = "°/s")]
    pub angular_velocity_cutoff: f32,

    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_controller_position_offset: [f32; 3],

    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_controller_rotation_offset: [f32; 3],

    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_hand_tracking_position_offset: [f32; 3],

    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_hand_tracking_rotation_offset: [f32; 3],

    #[schema(flag = "steamvr-restart")]
    pub override_trigger_threshold: Switch<ControllersTriggerOverrideDesc>,

    #[schema(flag = "steamvr-restart")]
    pub override_grip_threshold: Switch<ControllersGripOverrideDesc>,

    pub haptics: Switch<HapticsConfig>,

    pub use_headset_tracking_system: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum PositionRecenteringMode {
    Disabled,
    LocalFloor,
    Local {
        #[schema(gui(slider(min = 0.0, max = 3.0)), suffix = "m")]
        view_height: f32,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum RotationRecenteringMode {
    Disabled,
    Yaw,
    Tilted,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HeadsetDesc {
    #[schema(flag = "steamvr-restart")]
    pub mode_idx: u64,

    #[schema(flag = "steamvr-restart")]
    pub universe_id: u64,

    #[schema(flag = "steamvr-restart")]
    pub serial_number: String,

    #[schema(flag = "steamvr-restart")]
    pub tracking_system_name: String,

    #[schema(flag = "steamvr-restart")]
    pub model_number: String,

    #[schema(flag = "steamvr-restart")]
    pub driver_version: String,

    #[schema(flag = "steamvr-restart")]
    pub manufacturer_name: String,

    #[schema(flag = "steamvr-restart")]
    pub render_model_name: String,

    #[schema(flag = "steamvr-restart")]
    pub registered_device_type: String,

    #[schema(flag = "steamvr-restart")]
    pub tracking_ref_only: bool,

    #[schema(flag = "steamvr-restart")]
    pub enable_vive_tracker_proxy: bool,

    #[schema(flag = "steamvr-restart")]
    pub controllers: Switch<ControllersDesc>,

    #[schema(strings(
        help = r#"Disabled: the playspace origin is determined by the room-scale guardian setup.
Local floor: the origin is on the floor and resets when long pressing the oculus button.
Local: the origin resets when long pressing the oculus button, and is calculated as an offset from the current head position."#
    ))]
    pub position_recentering_mode: PositionRecenteringMode,

    #[schema(strings(
        help = r#"Disabled: the playspace orientation is determined by the room-scale guardian setup.
Yaw: the forward direction is reset when long pressing the oculus button.
Tilted: the world gets tilted when long pressing the oculus button. This is useful for using VR while laying down."#
    ))]
    pub rotation_recentering_mode: RotationRecenteringMode,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum SocketProtocol {
    #[schema(strings(display_name = "UDP"))]
    Udp,
    #[schema(strings(display_name = "TCP"))]
    Tcp,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DiscoveryConfig {
    #[schema(strings(
        help = "Allow untrusted clients to connect without confirmation. This is not recommended for security reasons."
    ))]
    pub auto_trust_clients: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum SocketBufferSize {
    Default,
    Maximum,
    Custom(#[schema(suffix = "B")] u32),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DisconnectionCriteria {
    #[schema(strings(display_name = "latency threshold"))]
    #[schema(gui(slider(min = 20, max = 1000, logarithmic)), suffix = "ms")]
    pub latency_threshold_ms: u64,

    #[schema(strings(display_name = "Sustain duration"), suffix = "s")]
    pub sustain_duration_s: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ConnectionDesc {
    pub client_discovery: Switch<DiscoveryConfig>,

    pub web_server_port: u16,

    #[schema(strings(
        help = r#"UDP: Faster, but less stable than TCP. Try this if your network is well optimized and free of interference.
TCP: Slower than UDP, but more stable. Pick this if you experience video or audio stutters with UDP."#
    ))]
    pub stream_protocol: SocketProtocol,

    #[schema(strings(display_name = "Server send buffer size"))]
    pub server_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Server receive buffer size"))]
    pub server_recv_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Client send buffer size"))]
    pub client_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "Client receive buffer size"))]
    pub client_recv_buffer_bytes: SocketBufferSize,

    pub stream_port: u16,

    #[schema(strings(
        help = "Reduce minimum delay between keyframes from 100ms to 5ms. Use on networks with high packet loss."
    ))]
    pub aggressive_keyframe_resend: bool,

    #[schema(strings(
        help = "This script will be ran when the headset connects. Env var ACTION will be set to `connect`."
    ))]
    pub on_connect_script: String,

    #[schema(strings(
        help = "This script will be ran when the headset disconnects, or when SteamVR shuts down. Env var ACTION will be set to `disconnect`."
    ))]
    pub on_disconnect_script: String,

    #[schema(gui(slider(min = 1024, max = 65507, logarithmic)), suffix = "B")]
    pub packet_size: i32,

    #[schema(suffix = " frames")]
    pub statistics_history_size: u64,

    pub disconnection_criteria: Switch<DisconnectionCriteria>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum DriverLaunchAction {
    UnregisterOtherDriversAtStartup,
    #[schema(strings(display_name = "Unregister ALVR at shutdown"))]
    UnregisterAlvrAtShutdown,
    NoAction,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    RawEvents,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Patches {
    #[schema(strings(
        help = "May cause jitter for Nvidia users. AMD users should keep this on. Must be off for Nvidia GPUs!",
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_reprojection: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ExtraDesc {
    #[schema(strings(help = "Write logs into the session_log.txt file."))]
    pub log_to_disk: bool,
    pub log_button_presses: bool,
    pub log_haptics: bool,
    pub save_video_stream: bool,

    #[schema(strings(
        help = r#"This controls the driver registration operations while launching SteamVR.
Unregister other drivers at startup: This is the recommended option and will handle most interferences from other installed drivers.
Unregister ALVR at shutdown: This should be used when you want to load other drivers like for full body tracking. Other VR streaming drivers like Virtual Desktop must be manually unregistered or uninstalled.
No action: All driver registration actions should be performed mnually, ALVR included. This allows to launch SteamVR without launching the dashboard first."#
    ))]
    pub driver_launch_action: DriverLaunchAction,

    pub notification_level: LogLevel,

    #[schema(flag = "steamvr-restart")]
    pub capture_frame_dir: String,

    pub open_setup_wizard: bool,

    pub patches: Patches,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub video: VideoDesc,
    pub audio: AudioSection,
    pub headset: HeadsetDesc,
    pub connection: ConnectionDesc,
    pub extra: ExtraDesc,
}

pub fn session_settings_default() -> SettingsDefault {
    SettingsDefault {
        video: VideoDescDefault {
            adapter_index: 0,
            render_resolution: FrameSizeDefault {
                variant: FrameSizeDefaultVariant::Scale,
                Scale: 0.75,
                Absolute: FrameSizeAbsoluteDefault {
                    width: 2880,
                    height: 1600,
                },
            },
            recommended_target_resolution: FrameSizeDefault {
                variant: FrameSizeDefaultVariant::Scale,
                Scale: 0.75,
                Absolute: FrameSizeAbsoluteDefault {
                    width: 2880,
                    height: 1600,
                },
            },
            preferred_fps: 72.,
            max_buffering_frames: 1.5,
            buffering_history_weight: 0.90,
            codec: CodecTypeDefault {
                variant: CodecTypeDefaultVariant::H264,
            },
            rate_control_mode: RateControlModeDefault {
                variant: RateControlModeDefaultVariant::Cbr,
            },
            filler_data: false,
            entropy_coding: EntropyCodingDefault {
                variant: EntropyCodingDefaultVariant::Cavlc,
            },
            use_10bit_encoder: false,
            force_sw_encoding: false,
            sw_thread_count: 0,
            bitrate: BitrateConfigDefault {
                mode: BitrateModeDefault {
                    ConstantMbps: 30,
                    Adaptive: BitrateModeAdaptiveDefault {
                        saturation_multiplier: 0.95,
                        max_bitrate_mbps: SwitchDefault {
                            enabled: false,
                            content: 100,
                        },
                        min_bitrate_mbps: SwitchDefault {
                            enabled: false,
                            content: 5,
                        },
                    },
                    variant: BitrateModeDefaultVariant::Adaptive,
                },
                framerate_reset_threshold_multiplier: 0.30,
                max_network_latency_ms: SwitchDefault {
                    enabled: false,
                    content: 8,
                },
                max_decoder_latency_ms: 15,
                decoder_latency_overstep_frames: 15,
                decoder_latency_overstep_multiplier: 0.99,
            },
            advanced_codec_options: AdvancedCodecOptionsDefault {
                encoder_quality_preset: EncoderQualityPresetDefault {
                    variant: EncoderQualityPresetDefaultVariant::Speed,
                },
                nvenc_overrides: NvencOverridesDefault {
                    tuning_preset: NvencTuningPresetDefault {
                        variant: NvencTuningPresetDefaultVariant::LowLatency,
                    },
                    multi_pass: NvencMultiPassDefault {
                        variant: NvencMultiPassDefaultVariant::QuarterResolution,
                    },
                    adaptive_quantization_mode: NvencAdaptiveQuantizationModeDefault {
                        variant: NvencAdaptiveQuantizationModeDefaultVariant::Spatial,
                    },
                    low_delay_key_frame_scale: -1,
                    refresh_rate: -1,
                    enable_intra_refresh: false,
                    intra_refresh_period: -1,
                    intra_refresh_count: -1,
                    max_num_ref_frames: -1,
                    gop_length: -1,
                    p_frame_strategy: -1,
                    rate_control_mode: -1,
                    rc_buffer_size: -1,
                    rc_initial_delay: -1,
                    rc_max_bitrate: -1,
                    rc_average_bitrate: -1,
                    enable_weighted_prediction: false,
                },
                amf_controls: AmfControlsDefault {
                    enable_vbaq: false,
                    use_preproc: false,
                    preproc_sigma: 4,
                    preproc_tor: 7,
                },
                mediacodec_extra_options: {
                    fn int32_default(int32: i32) -> MediacodecDataTypeDefault {
                        MediacodecDataTypeDefault {
                            variant: MediacodecDataTypeDefaultVariant::Int32,
                            Float: 0.0,
                            Int32: int32,
                            Int64: 0,
                            String: "".into(),
                        }
                    }
                    DictionaryDefault {
                        key: "".into(),
                        value: int32_default(0),
                        content: vec![
                            ("operating-rate".into(), int32_default(i32::MAX)),
                            ("priority".into(), int32_default(0)),
                            // low-latency: only applicable on API level 30. Quest 1 and 2 might not be
                            // cabable, since they are on level 29.
                            ("low-latency".into(), int32_default(1)),
                            (
                                "vendor.qti-ext-dec-low-latency.enable".into(),
                                int32_default(1),
                            ),
                        ],
                    }
                },
            },
            seconds_from_vsync_to_photons: 0.005,
            foveated_rendering: SwitchDefault {
                enabled: true,
                content: FoveatedRenderingDescDefault {
                    center_size_x: 0.4,
                    center_size_y: 0.35,
                    center_shift_x: 0.4,
                    center_shift_y: 0.1,
                    edge_ratio_x: 4.,
                    edge_ratio_y: 5.,
                },
            },
            oculus_foveation_level: OculusFovetionLevelDefault {
                variant: OculusFovetionLevelDefaultVariant::HighTop,
            },
            dynamic_oculus_foveation: true,
            color_correction: SwitchDefault {
                enabled: true,
                content: ColorCorrectionDescDefault {
                    brightness: 0.,
                    contrast: 0.,
                    saturation: 0.5,
                    gamma: 1.,
                    sharpening: 0.,
                },
            },
        },
        audio: AudioSectionDefault {
            linux_backend: LinuxAudioBackendDefault {
                variant: LinuxAudioBackendDefaultVariant::Alsa,
            },
            game_audio: SwitchDefault {
                enabled: !cfg!(target_os = "linux"),
                content: GameAudioDescDefault {
                    device_id: AudioDeviceIdDefault {
                        variant: AudioDeviceIdDefaultVariant::Default,
                        Name: "".into(),
                        Index: 1,
                    },
                    mute_when_streaming: true,
                    buffering_config: AudioBufferingConfigDefault {
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
            microphone: SwitchDefault {
                enabled: false,
                content: MicrophoneDescDefault {
                    input_device_id: AudioDeviceIdDefault {
                        variant: AudioDeviceIdDefaultVariant::Default,
                        Name: "".into(),
                        Index: 1,
                    },
                    #[cfg(not(target_os = "linux"))]
                    output_device_id: AudioDeviceIdDefault {
                        variant: AudioDeviceIdDefaultVariant::Default,
                        Name: "".into(),
                        Index: 1,
                    },
                    buffering_config: AudioBufferingConfigDefault {
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
        },
        headset: HeadsetDescDefault {
            mode_idx: 2,
            universe_id: 2,
            serial_number: "1WMGH000XX0000".into(),
            tracking_system_name: "oculus".into(),
            model_number: "Miramar".into(),
            driver_version: "1.55.0".into(),
            manufacturer_name: "Oculus".into(),
            render_model_name: "generic_hmd".into(),
            registered_device_type: "oculus/1WMGH000XX0000".into(),
            tracking_ref_only: false,
            enable_vive_tracker_proxy: false,
            controllers: SwitchDefault {
                enabled: true,
                content: ControllersDescDefault {
                    mode_idx: 7,
                    tracking_system_name: "oculus".into(),
                    manufacturer_name: "Oculus".into(),
                    model_number: "Miramar".into(),
                    render_model_name_left: "oculus_quest2_controller_left".into(),
                    render_model_name_right: "oculus_quest2_controller_right".into(),
                    serial_number: "1WMGH000XX0000_Controller".into(),
                    ctrl_type_left: "oculus_touch".into(),
                    ctrl_type_right: "oculus_touch".into(),
                    registered_device_type: "oculus/1WMGH000XX0000_Controller".into(),
                    input_profile_path: "{oculus}/input/touch_profile.json".into(),
                    pose_time_offset_ms: 20,
                    linear_velocity_cutoff: 0.05,
                    angular_velocity_cutoff: 10.0,
                    left_controller_position_offset: [0.0, 0.0, -0.11],
                    left_controller_rotation_offset: [-20.0, 0.0, 0.0],
                    left_hand_tracking_position_offset: [0.04, -0.02, -0.13],
                    left_hand_tracking_rotation_offset: [0.0, -45.0, -90.0],
                    override_trigger_threshold: SwitchDefault {
                        enabled: false,
                        content: ControllersTriggerOverrideDescDefault {
                            trigger_threshold: 0.1,
                        },
                    },
                    override_grip_threshold: SwitchDefault {
                        enabled: false,
                        content: ControllersGripOverrideDescDefault {
                            grip_threshold: 0.1,
                        },
                    },
                    haptics: SwitchDefault {
                        enabled: true,
                        content: HapticsConfigDefault {
                            intensity_multiplier: 1.,
                            amplitude_curve: 0.4,
                            min_duration_s: 0.01,
                            low_duration_amplitude_multiplier: 2.5,
                            low_duration_range_multiplier: 0.5,
                        },
                    },
                    use_headset_tracking_system: false,
                },
            },
            position_recentering_mode: PositionRecenteringModeDefault {
                Local: PositionRecenteringModeLocalDefault { view_height: 1.5 },
                variant: PositionRecenteringModeDefaultVariant::LocalFloor,
            },
            rotation_recentering_mode: RotationRecenteringModeDefault {
                variant: RotationRecenteringModeDefaultVariant::Yaw,
            },
        },
        connection: {
            let socket_buffer = SocketBufferSizeDefault {
                Custom: 100000,
                variant: SocketBufferSizeDefaultVariant::Maximum,
            };
            ConnectionDescDefault {
                client_discovery: SwitchDefault {
                    enabled: true,
                    content: DiscoveryConfigDefault {
                        auto_trust_clients: cfg!(debug_assertions),
                    },
                },
                web_server_port: 8082,
                stream_protocol: SocketProtocolDefault {
                    variant: SocketProtocolDefaultVariant::Udp,
                },
                server_send_buffer_bytes: socket_buffer.clone(),
                server_recv_buffer_bytes: socket_buffer.clone(),
                client_send_buffer_bytes: socket_buffer.clone(),
                client_recv_buffer_bytes: socket_buffer,
                stream_port: 9944,
                aggressive_keyframe_resend: false,
                on_connect_script: "".into(),
                on_disconnect_script: "".into(),
                packet_size: 1400,
                statistics_history_size: 256,
                disconnection_criteria: SwitchDefault {
                    enabled: false,
                    content: DisconnectionCriteriaDefault {
                        latency_threshold_ms: 150,
                        sustain_duration_s: 3,
                    },
                },
            }
        },
        extra: ExtraDescDefault {
            log_to_disk: cfg!(debug_assertions),
            log_button_presses: false,
            log_haptics: false,
            save_video_stream: false,
            driver_launch_action: DriverLaunchActionDefault {
                variant: DriverLaunchActionDefaultVariant::UnregisterOtherDriversAtStartup,
            },
            notification_level: LogLevelDefault {
                variant: if cfg!(debug_assertions) {
                    LogLevelDefaultVariant::Debug
                } else {
                    LogLevelDefaultVariant::Info
                },
            },
            capture_frame_dir: if !cfg!(target_os = "linux") {
                "/tmp".into()
            } else {
                "".into()
            },
            patches: PatchesDefault {
                linux_async_reprojection: false,
            },
            open_setup_wizard: alvr_common::is_stable() || alvr_common::is_nightly(),
        },
    }
}
