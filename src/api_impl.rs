use crate::types::{BatteryInfo, CommandResult, CommandState, KachakaError, Pose};
use crate::KachakaApiError;
use crate::{kachaka_api, StartCommandOptions};
use kachaka_api::kachaka_api_client::KachakaApiClient as TonicKachakaApiClient;
use tonic::transport::Channel;

fn parse_rpc_response_with_result<T>(
    response_result: std::result::Result<tonic::Response<T>, tonic::Status>,
    get_result: impl Fn(&T) -> Option<kachaka_api::Result>,
) -> Result<T, KachakaApiError> {
    match response_result {
        Ok(response) => {
            if let Some(result) = get_result(response.get_ref()) {
                if result.success {
                    Ok(response.into_inner())
                } else {
                    Err(KachakaApiError::ApiError(KachakaError {
                        error_code: result.error_code,
                    }))
                }
            } else {
                Err(KachakaApiError::NullResult)
            }
        }
        Err(e) => Err(KachakaApiError::CommunicationError(e)),
    }
}

fn parse_getter_response<T>(
    maybe_response: std::result::Result<tonic::Response<T>, tonic::Status>,
) -> Result<T, KachakaApiError> {
    match maybe_response {
        Ok(response) => Ok(response.into_inner()),
        Err(e) => Err(KachakaApiError::CommunicationError(e)),
    }
}

// getter api
pub async fn get_robot_serial_number(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<String, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_serial_number(request).await;
    parse_getter_response(response).map(|response| response.serial_number)
}

pub async fn get_robot_version(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<String, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_version(request).await;
    parse_getter_response(response).map(|response| response.version)
}

pub async fn get_robot_pose(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Pose, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_pose(request).await;
    let pose_result = parse_getter_response(response)?;
    if let Some(pose) = pose_result.pose {
        Ok(pose.into())
    } else {
        Err(KachakaApiError::NullResult)
    }
}

pub async fn get_battery_info(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<BatteryInfo, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_battery_info(request).await;
    parse_getter_response(response).map(|response| BatteryInfo {
        power_supply_status: response.power_supply_status.into(),
        remaining_percentage: response.remaining_percentage,
    })
}

pub async fn get_command_state(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<CommandState, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_command_state(request).await;
    parse_getter_response(response).map(|response| response.into())
}

async fn get_last_command_result_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, Option<CommandResult>), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_last_command_result(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.into()))
}

pub async fn get_last_command_result(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Option<CommandResult>, KachakaApiError> {
    get_last_command_result_with_cursor(client, cursor)
        .await
        .map(|(_, result)| result)
}

// command api
async fn start_command(
    client: &mut TonicKachakaApiClient<Channel>,
    command: kachaka_api::command::Command,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::StartCommandRequest {
        command: Some(kachaka_api::Command {
            command: Some(command),
        }),
        cancel_all: options.cancel_all,
        deferrable: options.deferrable,
        lock_on_end: options.lock_on_end,
        title: options.title,
        tts_on_success: options.tts_on_success,
    });
    let response = client.start_command(request).await;
    parse_rpc_response_with_result(
        response,
        |rpc_response: &kachaka_api::StartCommandResponse| rpc_response.result,
    )
    .map(|response| response.command_id)
}

pub async fn move_shelf(
    client: &mut TonicKachakaApiClient<Channel>,
    shelf_id: &str,
    location_id: &str,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::MoveShelfCommand(kachaka_api::MoveShelfCommand {
            target_shelf_id: shelf_id.to_string(),
            destination_location_id: location_id.to_string(),
        }),
        options,
    )
    .await
}

pub async fn return_shelf(
    client: &mut TonicKachakaApiClient<Channel>,
    shelf_id: &str,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::ReturnShelfCommand(kachaka_api::ReturnShelfCommand {
            target_shelf_id: shelf_id.to_string(),
        }),
        options,
    )
    .await
}

pub async fn undock_shelf(
    client: &mut TonicKachakaApiClient<Channel>,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::UndockShelfCommand(kachaka_api::UndockShelfCommand {
            target_shelf_id: "".to_string(),
        }),
        options,
    )
    .await
}

pub async fn move_to_location(
    client: &mut TonicKachakaApiClient<Channel>,
    location_id: &str,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::MoveToLocationCommand(kachaka_api::MoveToLocationCommand {
            target_location_id: location_id.to_string(),
        }),
        options,
    )
    .await
}

pub async fn return_home(
    client: &mut TonicKachakaApiClient<Channel>,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::ReturnHomeCommand(kachaka_api::ReturnHomeCommand {}),
        options,
    )
    .await
}

pub async fn dock_shelf(
    client: &mut TonicKachakaApiClient<Channel>,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::DockShelfCommand(kachaka_api::DockShelfCommand {}),
        options,
    )
    .await
}

pub async fn speak(
    client: &mut TonicKachakaApiClient<Channel>,
    text: &str,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::SpeakCommand(kachaka_api::SpeakCommand {
            text: text.to_string(),
        }),
        options,
    )
    .await
}

pub async fn move_to_pose(
    client: &mut TonicKachakaApiClient<Channel>,
    x: f64,
    y: f64,
    yaw: f64,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::MoveToPoseCommand(kachaka_api::MoveToPoseCommand {
            x,
            y,
            yaw,
        }),
        options,
    )
    .await
}

pub async fn lock(
    client: &mut TonicKachakaApiClient<Channel>,
    duration_sec: f64,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::LockCommand(kachaka_api::LockCommand { duration_sec }),
        options,
    )
    .await
}

pub async fn move_forward(
    client: &mut TonicKachakaApiClient<Channel>,
    distance_meter: f64,
    speed: f64,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::MoveForwardCommand(kachaka_api::MoveForwardCommand {
            distance_meter,
            speed,
        }),
        options,
    )
    .await
}

pub async fn rotate_in_place(
    client: &mut TonicKachakaApiClient<Channel>,
    angle_radian: f64,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::RotateInPlaceCommand(kachaka_api::RotateInPlaceCommand {
            angle_radian,
        }),
        options,
    )
    .await
}

pub async fn dock_any_shelf_with_registration(
    client: &mut TonicKachakaApiClient<Channel>,
    location_id: &str,
    options: StartCommandOptions,
) -> Result<String, KachakaApiError> {
    start_command(
        client,
        kachaka_api::command::Command::DockAnyShelfWithRegistrationCommand(
            kachaka_api::DockAnyShelfWithRegistrationCommand {
                dock_forward: true,
                target_location_id: location_id.to_string(),
            },
        ),
        options,
    )
    .await
}

pub async fn cancel_command(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<(), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::EmptyRequest {});
    let response = client.cancel_command(request).await;
    parse_rpc_response_with_result(
        response,
        |rpc_response: &kachaka_api::CancelCommandResponse| rpc_response.result,
    )
    .map(|_response| ())
}

pub async fn proceed(client: &mut TonicKachakaApiClient<Channel>) -> Result<(), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::EmptyRequest {});
    let response = client.proceed(request).await;
    parse_rpc_response_with_result(response, |rpc_response: &kachaka_api::ProceedResponse| {
        rpc_response.result
    })
    .map(|_response| ())
}
