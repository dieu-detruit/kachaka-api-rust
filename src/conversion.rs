use crate::kachaka_api;
use crate::types::{CommandResult, CommandState, KachakaError, Pose, PowerSupplyStatus};

impl From<kachaka_api::Result> for std::result::Result<(), KachakaError> {
    fn from(result: kachaka_api::Result) -> Self {
        if result.success {
            Ok(())
        } else {
            Err(KachakaError {
                error_code: result.error_code,
            })
        }
    }
}

impl From<kachaka_api::Pose> for Pose {
    fn from(pose: kachaka_api::Pose) -> Self {
        Pose {
            x: pose.x,
            y: pose.y,
            theta: pose.theta,
        }
    }
}

impl From<kachaka_api::PowerSupplyStatus> for PowerSupplyStatus {
    fn from(status: kachaka_api::PowerSupplyStatus) -> Self {
        match status {
            kachaka_api::PowerSupplyStatus::Charging => PowerSupplyStatus::Charging,
            kachaka_api::PowerSupplyStatus::Discharging => PowerSupplyStatus::Discharging,
            _ => panic!("Invalid power supply status"),
        }
    }
}

impl From<i32> for PowerSupplyStatus {
    fn from(status: i32) -> Self {
        kachaka_api::PowerSupplyStatus::try_from(status)
            .unwrap()
            .into()
    }
}

impl From<kachaka_api::GetCommandStateResponse> for CommandState {
    fn from(get_command_state_response: kachaka_api::GetCommandStateResponse) -> Self {
        match kachaka_api::CommandState::try_from(get_command_state_response.state).unwrap() {
            kachaka_api::CommandState::Unspecified => CommandState::Unspecified,
            kachaka_api::CommandState::Pending => CommandState::Pending,
            kachaka_api::CommandState::Running => CommandState::Running(
                get_command_state_response.command.unwrap(),
                get_command_state_response.command_id,
            ),
        }
    }
}

impl From<kachaka_api::GetLastCommandResultResponse> for Option<CommandResult> {
    fn from(response: kachaka_api::GetLastCommandResultResponse) -> Self {
        response.result.and_then(|result| {
            response.command.map(|command| CommandResult {
                command,
                result: result.into(),
            })
        })
    }
}
