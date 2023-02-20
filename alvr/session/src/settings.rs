use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{DictionaryDefault, SettingsSchema, Switch, SwitchDefault};

include!(concat!(env!("OUT_DIR"), "/openvr_property_keys.rs"));

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum FrameSize {
    Scale(#[schema(min = 0.25, max = 2., step = 0.01)] f32),
    Absolute {
        #[schema(min = 32, step = 32)]
        width: u32,
        #[schema(min = 32, step = 32)]
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
pub enum RateControlMode {
    #[schema(strings(display_name = "CBR"))]
    Cbr = 0,
    #[schema(strings(display_name = "VBR"))]
    Vbr = 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum EntropyCoding {
    #[schema(strings(display_name = "CABAC"))]
    Cabac = 0,
    #[schema(strings(display_name = "CAVLC"))]
    Cavlc = 1,
}

/// Except for preset, the value of these fields is not applied if == -1 (flag)
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct NvencOverrides {
    pub tuning_preset: NvencTuningPreset,
    pub multi_pass: NvencMultiPass,
    pub adaptive_quantization_mode: NvencAdaptiveQuantizationMode,
    pub low_delay_key_frame_scale: i64,
    pub refresh_rate: i64,
    pub enable_intra_refresh: bool,
    pub intra_refresh_period: i64,
    pub intra_refresh_count: i64,
    pub max_num_ref_frames: i64,
    pub gop_length: i64,
    pub p_frame_strategy: i64,
    pub rate_control_mode: i64,
    pub rc_buffer_size: i64,
    pub rc_initial_delay: i64,
    pub rc_max_bitrate: i64,
    pub rc_average_bitrate: i64,
    pub enable_weighted_prediction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AmfControls {
    pub enable_vbaq: bool,
    pub use_preproc: bool,
    #[schema(min = 0, max = 10)]
    pub preproc_sigma: u32,
    #[schema(min = 0, max = 10)]
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
    pub encoder_quality_preset: EncoderQualityPreset,
    pub nvenc_overrides: NvencOverrides,
    pub amf_controls: AmfControls,
    pub mediacodec_extra_options: Vec<(String, MediacodecDataType)>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum BitrateMode {
    ConstantMbps(#[schema(min = 1, max = 1000)] u64),
    Adaptive {
        #[schema(min = 0.5, max = 2.0, step = 0.05)]
        saturation_multiplier: f32,

        #[schema(min = 1, max = 1000, step = 1)]
        max_bitrate_mbps: Switch<u64>,

        #[schema(min = 1, max = 1000, step = 1)]
        min_bitrate_mbps: Switch<u64>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct BitrateConfig {
    pub mode: BitrateMode,

    #[schema(min = 0.01, max = 2.0, step = 0.01)]
    pub framerate_reset_threshold_multiplier: f32,

    #[schema(min = 1, max = 50, step = 1)]
    pub max_network_latency_ms: Switch<u64>,

    #[schema(min = 1, max = 50)]
    pub max_decoder_latency_ms: u64,

    #[schema(min = 1, max = 100)]
    pub decoder_latency_overstep_frames: u64,

    #[schema(min = 0.5, max = 1.0)]
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
    #[schema(
        strings(display_name = "Center region width"),
        min = 0.,
        max = 1.,
        step = 0.01
    )]
    pub center_size_x: f32,

    #[schema(
        strings(display_name = "Center region height"),
        min = 0.,
        max = 1.,
        step = 0.01
    )]
    pub center_size_y: f32,

    #[schema(strings(display_name = "Center shift X"), min = -1., max = 1., step = 0.01)]
    pub center_shift_x: f32,

    #[schema(strings(display_name = "Center shift Y"), min = -1., max = 1., step = 0.01)]
    pub center_shift_y: f32,

    #[schema(
        strings(display_name = "Horizontal edge ratio"),
        min = 1.,
        max = 10.,
        step = 1.
    )]
    pub edge_ratio_x: f32,

    #[schema(
        strings(display_name = "Vertical edge ratio"),
        min = 1.,
        max = 10.,
        step = 1.
    )]
    pub edge_ratio_y: f32,
}

#[repr(C)]
#[derive(SettingsSchema, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct ColorCorrectionDesc {
    #[schema(min = -1., max = 1., step = 0.01)]
    pub brightness: f32,

    #[schema(min = -1., max = 1., step = 0.01)]
    pub contrast: f32,

    #[schema(min = -1., max = 1., step = 0.01)]
    pub saturation: f32,

    #[schema(min = 0., max = 5., step = 0.01)]
    pub gamma: f32,

    #[schema(min = -1., max = 5., step = 0.01)]
    pub sharpening: f32,
}

// Note: This enum cannot be converted to camelCase due to a inconsistency between generation and
// validation: "hevc" vs "hEVC".
// This is caused by serde and settings-schema using different libraries for casing conversion
// todo: don't use casing conversion also for all other structs and enums
#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone)]
pub enum CodecType {
    #[schema(strings(display_name = "h264"))]
    H264,
    #[schema(strings(display_name = "HEVC"))]
    Hevc,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct VideoDesc {
    pub adapter_index: u32,

    pub render_resolution: FrameSize,

    pub recommended_target_resolution: FrameSize,

    #[schema(strings(display_name = "Preferred FPS"), min = 0.0)]
    pub preferred_fps: f32,

    #[schema(
        strings(display_name = "Maximum buffering (frames)"),
        min = 1.,
        max = 10.0,
        step = 0.1
    )]
    pub max_buffering_frames: f32,

    #[schema(min = 0.50, max = 0.99, step = 0.01)]
    pub buffering_history_weight: f32,

    pub codec: CodecType,

    pub rate_control_mode: RateControlMode,

    pub filler_data: bool,

    pub entropy_coding: EntropyCoding,

    pub use_10bit_encoder: bool,

    pub force_sw_encoding: bool,

    pub sw_thread_count: u32,

    pub bitrate: BitrateConfig,

    pub advanced_codec_options: AdvancedCodecOptions,

    pub seconds_from_vsync_to_photons: f32,

    pub foveated_rendering: Switch<FoveatedRenderingDesc>,

    pub oculus_foveation_level: OculusFovetionLevel,

    pub dynamic_oculus_foveation: bool,

    pub color_correction: Switch<ColorCorrectionDesc>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum AudioDeviceId {
    Default,
    Name(String),
    Index(#[schema(min = 1, gui = "up_down")] u64),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioBufferingConfig {
    #[schema(strings(display_name = "Average buffering (ms)"), min = 0, max = 200)]
    pub average_buffering_ms: u64,

    #[schema(strings(display_name = "Batch (ms)"), min = 1, max = 20)]
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
    #[schema(strings(display_name = "Output device ID"))]
    pub output_device_id: AudioDeviceId,
    pub buffering_config: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum LinuxAudioBackend {
    #[schema(strings(display_name = "ALSA"))]
    Alsa,

    Jack,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioSection {
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
    #[schema(min = 0.01, max = 1., step = 0.01)]
    pub trigger_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ControllersGripOverrideDesc {
    #[schema(min = 0.01, max = 1., step = 0.01)]
    pub grip_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HapticsConfig {
    #[schema(min = 0., max = 5., step = 0.1)]
    pub intensity_multiplier: f32,

    #[schema(min = 0., max = 1., step = 0.01)]
    pub amplitude_curve: f32,

    #[schema(min = 0., max = 0.1, step = 0.001)]
    pub min_duration_s: f32,

    #[schema(min = 1., max = 5., step = 0.1)]
    pub low_duration_amplitude_multiplier: f32,

    #[schema(min = 0., max = 1., step = 0.01)]
    pub low_duration_range_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ControllersDesc {
    pub mode_idx: i32,

    pub tracking_system_name: String,

    pub manufacturer_name: String,

    pub model_number: String,

    pub render_model_name_left: String,

    pub render_model_name_right: String,

    pub serial_number: String,

    pub ctrl_type_left: String,

    pub ctrl_type_right: String,

    pub registered_device_type: String,

    pub input_profile_path: String,

    #[schema(min = -50, max = 50, step = 1)]
    pub pose_time_offset_ms: i64,

    #[schema(min = 0., max = 1.0, step = 0.001)]
    pub linear_velocity_cutoff: f32,

    #[schema(min = 0., max = 100., step = 1.)]
    pub angular_velocity_cutoff: f32,

    pub left_controller_position_offset: [f32; 3],

    pub left_controller_rotation_offset: [f32; 3],

    pub left_hand_tracking_position_offset: [f32; 3],

    pub left_hand_tracking_rotation_offset: [f32; 3],

    pub override_trigger_threshold: Switch<ControllersTriggerOverrideDesc>,

    pub override_grip_threshold: Switch<ControllersGripOverrideDesc>,

    pub haptics: Switch<HapticsConfig>,

    pub use_headset_tracking_system: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum PositionRecenteringMode {
    Disabled,
    LocalFloor,

    Local { view_height: f32 },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum RotationRecenteringMode {
    Disabled,
    Yaw,
    Tilted,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HeadsetDesc {
    pub mode_idx: u64,

    pub universe_id: u64,

    pub serial_number: String,

    pub tracking_system_name: String,

    pub model_number: String,

    pub driver_version: String,

    pub manufacturer_name: String,

    pub render_model_name: String,

    pub registered_device_type: String,

    pub tracking_ref_only: bool,

    pub enable_vive_tracker_proxy: bool,

    pub controllers: Switch<ControllersDesc>,

    pub position_recentering_mode: PositionRecenteringMode,

    pub rotation_recentering_mode: RotationRecenteringMode,

    pub extra_latency_mode: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum SocketProtocol {
    Udp,
    Tcp,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DiscoveryConfig {
    pub auto_trust_clients: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum SocketBufferSize {
    Default,
    Maximum,
    Custom(u32),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectionCriteria {
    pub latency_threshold_ms: u64,
    pub sustain_duration_s: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDesc {
    pub client_discovery: Switch<DiscoveryConfig>,

    #[schema(min = 1024, max = 0xFFFF)]
    pub web_server_port: u16,

    pub stream_protocol: SocketProtocol,

    pub server_send_buffer_bytes: SocketBufferSize,

    pub server_recv_buffer_bytes: SocketBufferSize,

    pub client_send_buffer_bytes: SocketBufferSize,

    pub client_recv_buffer_bytes: SocketBufferSize,

    pub stream_port: u16,

    pub aggressive_keyframe_resend: bool,

    pub on_connect_script: String,

    pub on_disconnect_script: String,

    // Max packet size is 64KB for TCP and 65507 bytes for UDP
    #[schema(min = 0, max = 0xFFFF)]
    pub packet_size: i32,

    pub statistics_history_size: u64,

    pub disconnection_criteria: Switch<DisconnectionCriteria>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Patches {
    pub remove_sync_popup: bool,
    pub linux_async_reprojection: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ExtraDesc {
    pub revert_confirm_dialog: bool,
    pub restart_confirm_dialog: bool,
    pub prompt_before_update: bool,
    pub log_to_disk: bool,
    pub log_button_presses: bool,
    pub log_haptics: bool,
    pub save_video_stream: bool,

    pub notification_level: LogLevel,
    pub exclude_notifications_without_id: bool,

    pub capture_frame_dir: String,

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
            extra_latency_mode: false,
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
            revert_confirm_dialog: true,
            restart_confirm_dialog: true,
            prompt_before_update: true,
            log_to_disk: cfg!(debug_assertions),
            log_button_presses: false,
            log_haptics: false,
            save_video_stream: false,
            notification_level: LogLevelDefault {
                variant: if cfg!(debug_assertions) {
                    LogLevelDefaultVariant::Info
                } else {
                    LogLevelDefaultVariant::Warning
                },
            },
            exclude_notifications_without_id: false,
            capture_frame_dir: if !cfg!(target_os = "linux") {
                "/tmp".into()
            } else {
                "".into()
            },
            patches: PatchesDefault {
                remove_sync_popup: false,
                linux_async_reprojection: false,
            },
        },
    }
}
