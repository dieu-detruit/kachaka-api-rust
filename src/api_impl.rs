use std::collections::HashMap;

use crate::types::{BatteryInfo, CommandResult, CommandState, KachakaError, Pose};
use crate::KachakaApiError;
use crate::{kachaka_api, StartCommandOptions};

use futures::stream::Stream;
use image::DynamicImage;
use kachaka_api::kachaka_api_client::KachakaApiClient as TonicKachakaApiClient;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
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

// GetRobotSerialNumber
async fn get_robot_serial_number_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, String), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_serial_number(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.serial_number))
}

pub async fn get_robot_serial_number(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<String, KachakaApiError> {
    get_robot_serial_number_with_cursor(client, cursor)
        .await
        .map(|(_, serial_number)| serial_number)
}

pub async fn get_latest_robot_serial_number(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<String, KachakaApiError> {
    get_robot_serial_number(client, 0).await
}

pub async fn watch_robot_serial_number(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<String, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<String, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_robot_serial_number_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, serial_number)) => {
                    cursor = new_cursor;
                    tx.send(Ok(serial_number)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetRobotVersion
async fn get_robot_version_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, String), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_version(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.version))
}

pub async fn get_robot_version(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<String, KachakaApiError> {
    get_robot_version_with_cursor(client, cursor)
        .await
        .map(|(_, version)| version)
}

pub async fn get_latest_robot_version(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<String, KachakaApiError> {
    get_robot_version(client, 0).await
}

pub async fn watch_robot_version(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<String, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<String, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_robot_version_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, version)) => {
                    cursor = new_cursor;
                    tx.send(Ok(version)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetRobotPose
async fn get_robot_pose_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, Pose), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_robot_pose(request).await;
    let pose_result = parse_getter_response(response)?;
    if let Some(pose) = pose_result.pose {
        Ok((pose_result.metadata.unwrap().cursor, pose.into()))
    } else {
        Err(KachakaApiError::NullResult)
    }
}

pub async fn get_robot_pose(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Pose, KachakaApiError> {
    get_robot_pose_with_cursor(client, cursor)
        .await
        .map(|(_, pose)| pose)
}

pub async fn get_latest_robot_pose(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<Pose, KachakaApiError> {
    get_robot_pose(client, 0).await
}

pub async fn watch_robot_pose(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<Pose, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Pose, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_robot_pose_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, pose)) => {
                    cursor = new_cursor;
                    tx.send(Ok(pose)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetBatteryInfo
async fn get_battery_info_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, BatteryInfo), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_battery_info(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            BatteryInfo {
                power_supply_status: response.power_supply_status.into(),
                remaining_percentage: response.remaining_percentage,
            },
        )
    })
}

pub async fn get_battery_info(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<BatteryInfo, KachakaApiError> {
    get_battery_info_with_cursor(client, cursor)
        .await
        .map(|(_, battery_info)| battery_info)
}

pub async fn get_latest_battery_info(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<BatteryInfo, KachakaApiError> {
    get_battery_info(client, 0).await
}

pub async fn watch_battery_info(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<BatteryInfo, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<BatteryInfo, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_battery_info_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, battery_info)) => {
                    cursor = new_cursor;
                    tx.send(Ok(battery_info)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetFrontCameraRosImage
async fn get_front_camera_ros_image_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, DynamicImage), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_front_camera_ros_image(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            DynamicImage::from(response.image.unwrap()),
        )
    })
}

pub async fn get_front_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<DynamicImage, KachakaApiError> {
    get_front_camera_ros_image_with_cursor(client, cursor)
        .await
        .map(|(_, image)| image)
}

pub async fn get_latest_front_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<DynamicImage, KachakaApiError> {
    get_front_camera_ros_image(client, 0).await
}

pub async fn watch_front_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<DynamicImage, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<DynamicImage, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_front_camera_ros_image_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, image)) => {
                    cursor = new_cursor;
                    tx.send(Ok(image)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetFrontCameraRosCompressedImage
async fn get_front_camera_ros_compressed_image_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, DynamicImage), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_front_camera_ros_compressed_image(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            DynamicImage::from(response.image.unwrap()),
        )
    })
}

pub async fn get_front_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<DynamicImage, KachakaApiError> {
    get_front_camera_ros_compressed_image_with_cursor(client, cursor)
        .await
        .map(|(_, image)| image)
}

pub async fn get_latest_front_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<DynamicImage, KachakaApiError> {
    get_front_camera_ros_compressed_image(client, 0).await
}

pub async fn watch_front_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<DynamicImage, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<DynamicImage, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_front_camera_ros_compressed_image_with_cursor(&mut client_clone, cursor).await
            {
                Ok((new_cursor, image)) => {
                    cursor = new_cursor;
                    tx.send(Ok(image)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetBackCameraRosImage
async fn get_back_camera_ros_image_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, DynamicImage), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_back_camera_ros_image(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            DynamicImage::from(response.image.unwrap()),
        )
    })
}

pub async fn get_back_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<DynamicImage, KachakaApiError> {
    get_back_camera_ros_image_with_cursor(client, cursor)
        .await
        .map(|(_, image)| image)
}

pub async fn get_latest_back_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<DynamicImage, KachakaApiError> {
    get_back_camera_ros_image(client, 0).await
}

pub async fn watch_back_camera_ros_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<DynamicImage, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<DynamicImage, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_back_camera_ros_image_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, image)) => {
                    cursor = new_cursor;
                    tx.send(Ok(image)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetBackCameraRosCompressedImage
async fn get_back_camera_ros_compressed_image_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, DynamicImage), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_back_camera_ros_compressed_image(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            DynamicImage::from(response.image.unwrap()),
        )
    })
}

pub async fn get_back_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<DynamicImage, KachakaApiError> {
    get_back_camera_ros_compressed_image_with_cursor(client, cursor)
        .await
        .map(|(_, image)| image)
}

pub async fn get_latest_back_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<DynamicImage, KachakaApiError> {
    get_back_camera_ros_compressed_image(client, 0).await
}

pub async fn watch_back_camera_ros_compressed_image(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<DynamicImage, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<DynamicImage, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_back_camera_ros_compressed_image_with_cursor(&mut client_clone, cursor).await
            {
                Ok((new_cursor, image)) => {
                    cursor = new_cursor;
                    tx.send(Ok(image)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetRobotErrorCodeJson
fn parse_robot_error_code_json(
    response: kachaka_api::GetRobotErrorCodeJsonResponse,
) -> Result<HashMap<i32, HashMap<String, String>>, KachakaApiError> {
    let items: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(&response.json).map_err(KachakaApiError::JsonParseError)?;
    let mut result = HashMap::new();
    for item in items {
        let key = item.get("code").unwrap().as_i64().unwrap() as i32;
        let mut map = HashMap::new();
        for (k, v) in item {
            if k == "code" {
                continue;
            }
            map.insert(k, v.as_str().unwrap().to_string());
        }
        result.insert(key, map);
    }
    Ok(result)
}

pub async fn get_robot_error_code_json(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<HashMap<i32, HashMap<String, String>>, KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::EmptyRequest {});
    let response = client.get_robot_error_code_json(request).await;
    match response {
        Ok(response) => {
            if let Some(result) = response.get_ref().result {
                if result.success {
                    Ok(parse_robot_error_code_json(response.into_inner())?)
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

// GetError
async fn get_error_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, Vec<KachakaError>), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_error(request).await;
    parse_getter_response(response).map(|response| {
        (
            response.metadata.unwrap().cursor,
            response
                .error_codes
                .into_iter()
                .map(|e| KachakaError { error_code: e })
                .collect(),
        )
    })
}

pub async fn get_error(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Vec<KachakaError>, KachakaApiError> {
    get_error_with_cursor(client, cursor)
        .await
        .map(|(_, errors)| errors)
}

pub async fn get_latest_error(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<Vec<KachakaError>, KachakaApiError> {
    get_error_with_cursor(client, 0)
        .await
        .map(|(_, errors)| errors)
}

pub async fn watch_error(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<Vec<KachakaError>, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Vec<KachakaError>, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_error_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, errors)) => {
                    cursor = new_cursor;
                    tx.send(Ok(errors)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetCommandState
async fn get_command_state_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, CommandState), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_command_state(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.into()))
}

pub async fn get_command_state(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<CommandState, KachakaApiError> {
    get_command_state_with_cursor(client, cursor)
        .await
        .map(|(_, command_state)| command_state)
}

pub async fn get_latest_command_state(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<CommandState, KachakaApiError> {
    get_command_state(client, 0).await
}

pub async fn watch_command_state(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<CommandState, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<CommandState, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_command_state_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, command_state)) => {
                    cursor = new_cursor;
                    tx.send(Ok(command_state)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });
    UnboundedReceiverStream::new(rx)
}

// GetLastCommandResult
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

pub async fn get_latest_last_command_result(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<Option<CommandResult>, KachakaApiError> {
    get_last_command_result(client, 0).await
}

pub async fn watch_last_command_result(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<Option<CommandResult>, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Option<CommandResult>, KachakaApiError>>();

    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_last_command_result_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, result)) => {
                    cursor = new_cursor;
                    tx.send(Ok(result)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });

    UnboundedReceiverStream::new(rx)
}

// command api
// StartCommand
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

// CancelCommand
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

// Proceed
pub async fn proceed(client: &mut TonicKachakaApiClient<Channel>) -> Result<(), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::EmptyRequest {});
    let response = client.proceed(request).await;
    parse_rpc_response_with_result(response, |rpc_response: &kachaka_api::ProceedResponse| {
        rpc_response.result
    })
    .map(|_response| ())
}

// GetLocations
async fn get_locations_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, Vec<kachaka_api::Location>), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_locations(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.locations))
}

pub async fn get_locations(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Vec<kachaka_api::Location>, KachakaApiError> {
    get_locations_with_cursor(client, cursor)
        .await
        .map(|(_, locations)| locations)
}

pub async fn get_latest_locations(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<Vec<kachaka_api::Location>, KachakaApiError> {
    get_locations(client, 0).await
}

pub async fn watch_locations(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<Vec<kachaka_api::Location>, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Vec<kachaka_api::Location>, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            match get_locations_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, locations)) => {
                    cursor = new_cursor;
                    tx.send(Ok(locations)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });

    UnboundedReceiverStream::new(rx)
}

// GetShelves
async fn get_shelves_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, Vec<kachaka_api::Shelf>), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_shelves(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.shelves))
}

pub async fn get_shelves(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<Vec<kachaka_api::Shelf>, KachakaApiError> {
    get_shelves_with_cursor(client, cursor)
        .await
        .map(|(_, shelves)| shelves)
}

pub async fn get_latest_shelves(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<Vec<kachaka_api::Shelf>, KachakaApiError> {
    get_shelves(client, 0).await
}

pub async fn watch_shelves(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<Vec<kachaka_api::Shelf>, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<Vec<kachaka_api::Shelf>, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();

    tokio::spawn(async move {
        loop {
            match get_shelves_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, shelves)) => {
                    cursor = new_cursor;
                    tx.send(Ok(shelves)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });

    UnboundedReceiverStream::new(rx)
}

// GetMovingShelfId
async fn get_moving_shelf_id_with_cursor(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<(i64, String), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::GetRequest {
        metadata: Some(kachaka_api::Metadata { cursor }),
    });
    let response = client.get_moving_shelf_id(request).await;
    parse_getter_response(response)
        .map(|response| (response.metadata.unwrap().cursor, response.shelf_id))
}

pub async fn get_moving_shelf_id(
    client: &mut TonicKachakaApiClient<Channel>,
    cursor: i64,
) -> Result<String, KachakaApiError> {
    get_moving_shelf_id_with_cursor(client, cursor)
        .await
        .map(|(_, shelf_id)| shelf_id)
}

pub async fn get_latest_moving_shelf_id(
    client: &mut TonicKachakaApiClient<Channel>,
) -> Result<String, KachakaApiError> {
    get_moving_shelf_id(client, 0).await
}

pub async fn watch_moving_shelf_id(
    client: &mut TonicKachakaApiClient<Channel>,
) -> impl Stream<Item = Result<String, KachakaApiError>> {
    let (tx, rx) = mpsc::unbounded_channel::<Result<String, KachakaApiError>>();
    let mut cursor = 0;
    let mut client_clone = client.clone();

    tokio::spawn(async move {
        loop {
            match get_moving_shelf_id_with_cursor(&mut client_clone, cursor).await {
                Ok((new_cursor, shelf_id)) => {
                    cursor = new_cursor;
                    tx.send(Ok(shelf_id)).unwrap();
                }
                Err(e) => {
                    tx.send(Err(e)).unwrap();
                }
            }
        }
    });

    UnboundedReceiverStream::new(rx)
}

// ResetShelfPose
pub async fn reset_shelf_pose(
    client: &mut TonicKachakaApiClient<Channel>,
    shelf_id: &str,
) -> Result<(), KachakaApiError> {
    let request = tonic::Request::new(kachaka_api::ResetShelfPoseRequest {
        shelf_id: shelf_id.to_string(),
    });
    let response = client.reset_shelf_pose(request).await;
    parse_rpc_response_with_result(
        response,
        |rpc_response: &kachaka_api::ResetShelfPoseResponse| rpc_response.result,
    )
    .map(|_response| ())
}
