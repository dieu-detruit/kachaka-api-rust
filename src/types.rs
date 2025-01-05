use crate::kachaka_api;

#[derive(Debug)]
pub struct KachakaError {
    pub error_code: i32,
}

#[derive(Debug)]
pub enum KachakaApiError {
    CommunicationError(tonic::Status),
    ApiError(KachakaError),
    NullResult,
    JsonParseError(serde_json::Error),
}

#[derive(Debug)]
pub struct Pose {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

#[derive(Debug)]
pub enum PowerSupplyStatus {
    Charging,
    Discharging,
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub power_supply_status: PowerSupplyStatus,
    pub remaining_percentage: f64,
}

#[derive(Debug)]
pub enum CommandState {
    Unspecified,
    Pending,
    Running(kachaka_api::Command, String),
}

#[derive(Debug)]
pub struct CommandResult {
    pub command: kachaka_api::Command,
    pub result: std::result::Result<(), KachakaError>,
}
