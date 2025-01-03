use kachaka_api::kachaka_api_client::KachakaApiClient as TonicKachakaApiClient;
use tonic::transport::Channel;

pub mod kachaka_api {
    tonic::include_proto!("kachaka_api");
}

pub mod conversion;
pub mod options;
pub mod types;

pub use options::StartCommandOptions;
pub use types::{BatteryInfo, CommandResult, CommandState, KachakaApiError, KachakaError, Pose};

#[derive(Clone)]
pub struct KachakaApiClient {
    client: TonicKachakaApiClient<Channel>,
}

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

impl KachakaApiClient {
    pub async fn connect<D>(target: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let client = TonicKachakaApiClient::connect(target).await?;
        Ok(Self { client })
    }

    // getter api
    pub async fn get_robot_serial_number(
        &mut self,
        cursor: i64,
    ) -> Result<String, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_robot_serial_number(request).await;
        parse_getter_response(response).map(|response| response.serial_number)
    }

    pub async fn get_robot_version(&mut self, cursor: i64) -> Result<String, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_robot_version(request).await;
        parse_getter_response(response).map(|response| response.version)
    }

    pub async fn get_robot_pose(&mut self, cursor: i64) -> Result<Pose, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_robot_pose(request).await;
        let pose_result = parse_getter_response(response)?;
        if let Some(pose) = pose_result.pose {
            Ok(pose.into())
        } else {
            Err(KachakaApiError::NullResult)
        }
    }

    pub async fn get_battery_info(&mut self, cursor: i64) -> Result<BatteryInfo, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_battery_info(request).await;
        parse_getter_response(response).map(|response| BatteryInfo {
            power_supply_status: response.power_supply_status.into(),
            remaining_percentage: response.remaining_percentage,
        })
    }

    pub async fn get_command_state(
        &mut self,
        cursor: i64,
    ) -> Result<CommandState, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_command_state(request).await;
        parse_getter_response(response).map(|response| response.into())
    }

    pub async fn get_last_command_result(
        &mut self,
        cursor: i64,
    ) -> Result<Option<CommandResult>, KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::GetRequest {
            metadata: Some(kachaka_api::Metadata { cursor }),
        });
        let response = self.client.get_last_command_result(request).await;
        parse_getter_response(response).map(|response| response.into())
    }

    // command api
    async fn start_command(
        &mut self,
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
        let response = self.client.start_command(request).await;
        parse_rpc_response_with_result(
            response,
            |rpc_response: &kachaka_api::StartCommandResponse| rpc_response.result,
        )
        .map(|response| response.command_id)
    }

    pub async fn move_shelf(
        &mut self,
        shelf_id: &str,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::MoveShelfCommand(kachaka_api::MoveShelfCommand {
                target_shelf_id: shelf_id.to_string(),
                destination_location_id: location_id.to_string(),
            }),
            options,
        )
        .await
    }

    pub async fn return_shelf(
        &mut self,
        shelf_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::ReturnShelfCommand(kachaka_api::ReturnShelfCommand {
                target_shelf_id: shelf_id.to_string(),
            }),
            options,
        )
        .await
    }

    pub async fn undock_shelf(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::UndockShelfCommand(kachaka_api::UndockShelfCommand {
                target_shelf_id: "".to_string(),
            }),
            options,
        )
        .await
    }

    pub async fn move_to_location(
        &mut self,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::MoveToLocationCommand(
                kachaka_api::MoveToLocationCommand {
                    target_location_id: location_id.to_string(),
                },
            ),
            options,
        )
        .await
    }

    pub async fn return_home(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::ReturnHomeCommand(kachaka_api::ReturnHomeCommand {}),
            options,
        )
        .await
    }

    pub async fn dock_shelf(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::DockShelfCommand(kachaka_api::DockShelfCommand {}),
            options,
        )
        .await
    }

    pub async fn speak(
        &mut self,
        text: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::SpeakCommand(kachaka_api::SpeakCommand {
                text: text.to_string(),
            }),
            options,
        )
        .await
    }

    pub async fn move_to_pose(
        &mut self,
        x: f64,
        y: f64,
        yaw: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
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
        &mut self,
        duration_sec: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::LockCommand(kachaka_api::LockCommand { duration_sec }),
            options,
        )
        .await
    }

    pub async fn move_forward(
        &mut self,
        distance_meter: f64,
        speed: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::MoveForwardCommand(kachaka_api::MoveForwardCommand {
                distance_meter,
                speed,
            }),
            options,
        )
        .await
    }

    pub async fn rotate_in_place(
        &mut self,
        angle_radian: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
            kachaka_api::command::Command::RotateInPlaceCommand(
                kachaka_api::RotateInPlaceCommand { angle_radian },
            ),
            options,
        )
        .await
    }

    pub async fn dock_any_shelf_with_registration(
        &mut self,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        self.start_command(
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

    pub async fn cancel_command(&mut self) -> Result<(), KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::EmptyRequest {});
        let response = self.client.cancel_command(request).await;
        parse_rpc_response_with_result(
            response,
            |rpc_response: &kachaka_api::CancelCommandResponse| rpc_response.result,
        )
        .map(|_response| ())
    }

    pub async fn proceed(&mut self) -> Result<(), KachakaApiError> {
        let request = tonic::Request::new(kachaka_api::EmptyRequest {});
        let response = self.client.proceed(request).await;
        parse_rpc_response_with_result(response, |rpc_response: &kachaka_api::ProceedResponse| {
            rpc_response.result
        })
        .map(|_response| ())
    }
}
