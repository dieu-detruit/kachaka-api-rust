use kachaka_api::kachaka_api_client::KachakaApiClient as TonicKachakaApiClient;
use tonic::transport::Channel;

pub mod kachaka_api {
    tonic::include_proto!("kachaka_api");
}

pub mod options;

pub use options::StartCommandOptions;

#[derive(Clone)]
pub struct KachakaApiClient {
    client: TonicKachakaApiClient<Channel>,
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

    async fn start_command(
        &mut self,
        command: kachaka_api::command::Command,
        options: StartCommandOptions,
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
        let response = self.client.start_command(request).await?;
        Ok(response.into_inner())
    }

    pub async fn move_shelf(
        &mut self,
        shelf_id: &str,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
        self.start_command(
            kachaka_api::command::Command::ReturnHomeCommand(kachaka_api::ReturnHomeCommand {}),
            options,
        )
        .await
    }

    pub async fn dock_shelf(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
        let request = tonic::Request::new(kachaka_api::StartCommandRequest {
            command: Some(kachaka_api::Command {
                command: Some(kachaka_api::command::Command::SpeakCommand(
                    kachaka_api::SpeakCommand {
                        text: text.to_string(),
                    },
                )),
            }),
            cancel_all: options.cancel_all,
            deferrable: options.deferrable,
            lock_on_end: options.lock_on_end,
            title: options.title,
            tts_on_success: options.tts_on_success,
        });
        let response = self.client.start_command(request).await?;
        Ok(response.into_inner())
    }

    pub async fn move_to_pose(
        &mut self,
        x: f64,
        y: f64,
        yaw: f64,
        options: StartCommandOptions,
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
        self.start_command(
            kachaka_api::command::Command::LockCommand(kachaka_api::LockCommand {
                duration_sec: duration_sec,
            }),
            options,
        )
        .await
    }

    pub async fn move_forward(
        &mut self,
        distance_meter: f64,
        speed: f64,
        options: StartCommandOptions,
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
    ) -> Result<kachaka_api::StartCommandResponse, tonic::Status> {
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
}
