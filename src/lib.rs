use futures::stream::Stream;
use kachaka_api::kachaka_api_client::KachakaApiClient as TonicKachakaApiClient;
use tonic::transport::Channel;
pub mod kachaka_api {
    tonic::include_proto!("kachaka_api");
}

pub mod api_impl;
pub mod conversion;
pub mod options;
pub mod types;

pub use options::StartCommandOptions;
pub use types::{BatteryInfo, CommandResult, CommandState, KachakaApiError, KachakaError, Pose};

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

    // getter api
    // GetRobotSerialNumber
    pub async fn get_robot_serial_number(
        &mut self,
        cursor: i64,
    ) -> Result<String, KachakaApiError> {
        api_impl::get_robot_serial_number(&mut self.client, cursor).await
    }

    pub async fn get_latest_robot_serial_number(&mut self) -> Result<String, KachakaApiError> {
        api_impl::get_latest_robot_serial_number(&mut self.client).await
    }

    pub async fn watch_robot_serial_number(
        &mut self,
    ) -> impl Stream<Item = Result<String, KachakaApiError>> {
        api_impl::watch_robot_serial_number(&mut self.client).await
    }

    // GetRobotVersion
    pub async fn get_robot_version(&mut self, cursor: i64) -> Result<String, KachakaApiError> {
        api_impl::get_robot_version(&mut self.client, cursor).await
    }

    pub async fn get_latest_robot_version(&mut self) -> Result<String, KachakaApiError> {
        api_impl::get_latest_robot_version(&mut self.client).await
    }

    pub async fn watch_robot_version(
        &mut self,
    ) -> impl Stream<Item = Result<String, KachakaApiError>> {
        api_impl::watch_robot_version(&mut self.client).await
    }

    // GetRobotPose
    pub async fn get_robot_pose(&mut self, cursor: i64) -> Result<Pose, KachakaApiError> {
        api_impl::get_robot_pose(&mut self.client, cursor).await
    }

    pub async fn get_latest_robot_pose(&mut self) -> Result<Pose, KachakaApiError> {
        api_impl::get_latest_robot_pose(&mut self.client).await
    }

    pub async fn watch_robot_pose(&mut self) -> impl Stream<Item = Result<Pose, KachakaApiError>> {
        api_impl::watch_robot_pose(&mut self.client).await
    }

    // GetBatteryInfo
    pub async fn get_battery_info(&mut self, cursor: i64) -> Result<BatteryInfo, KachakaApiError> {
        api_impl::get_battery_info(&mut self.client, cursor).await
    }

    pub async fn get_latest_battery_info(&mut self) -> Result<BatteryInfo, KachakaApiError> {
        api_impl::get_latest_battery_info(&mut self.client).await
    }

    pub async fn watch_battery_info(
        &mut self,
    ) -> impl Stream<Item = Result<BatteryInfo, KachakaApiError>> {
        api_impl::watch_battery_info(&mut self.client).await
    }

    // GetCommandState
    pub async fn get_command_state(
        &mut self,
        cursor: i64,
    ) -> Result<CommandState, KachakaApiError> {
        api_impl::get_command_state(&mut self.client, cursor).await
    }

    pub async fn get_latest_command_state(&mut self) -> Result<CommandState, KachakaApiError> {
        api_impl::get_latest_command_state(&mut self.client).await
    }

    pub async fn watch_command_state(
        &mut self,
    ) -> impl Stream<Item = Result<CommandState, KachakaApiError>> {
        api_impl::watch_command_state(&mut self.client).await
    }

    // GetLastCommandResult
    pub async fn get_last_command_result(
        &mut self,
        cursor: i64,
    ) -> Result<Option<CommandResult>, KachakaApiError> {
        api_impl::get_last_command_result(&mut self.client, cursor).await
    }

    pub async fn watch_last_command_result(
        &mut self,
    ) -> impl Stream<Item = Result<Option<CommandResult>, KachakaApiError>> {
        api_impl::watch_last_command_result(&mut self.client).await
    }

    // command api
    pub async fn move_shelf(
        &mut self,
        shelf_id: &str,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::move_shelf(&mut self.client, shelf_id, location_id, options).await
    }

    pub async fn return_shelf(
        &mut self,
        shelf_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::return_shelf(&mut self.client, shelf_id, options).await
    }

    pub async fn undock_shelf(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::undock_shelf(&mut self.client, options).await
    }

    pub async fn move_to_location(
        &mut self,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::move_to_location(&mut self.client, location_id, options).await
    }

    pub async fn return_home(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::return_home(&mut self.client, options).await
    }

    pub async fn dock_shelf(
        &mut self,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::dock_shelf(&mut self.client, options).await
    }

    pub async fn speak(
        &mut self,
        text: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::speak(&mut self.client, text, options).await
    }

    pub async fn move_to_pose(
        &mut self,
        x: f64,
        y: f64,
        yaw: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::move_to_pose(&mut self.client, x, y, yaw, options).await
    }

    pub async fn lock(
        &mut self,
        duration_sec: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::lock(&mut self.client, duration_sec, options).await
    }

    pub async fn move_forward(
        &mut self,
        distance_meter: f64,
        speed: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::move_forward(&mut self.client, distance_meter, speed, options).await
    }

    pub async fn rotate_in_place(
        &mut self,
        angle_radian: f64,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::rotate_in_place(&mut self.client, angle_radian, options).await
    }

    pub async fn dock_any_shelf_with_registration(
        &mut self,
        location_id: &str,
        options: StartCommandOptions,
    ) -> Result<String, KachakaApiError> {
        api_impl::dock_any_shelf_with_registration(&mut self.client, location_id, options).await
    }

    pub async fn cancel_command(&mut self) -> Result<(), KachakaApiError> {
        api_impl::cancel_command(&mut self.client).await
    }

    pub async fn proceed(&mut self) -> Result<(), KachakaApiError> {
        api_impl::proceed(&mut self.client).await
    }
}
