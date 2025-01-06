use crate::kachaka_api;
use crate::types::{CommandResult, CommandState, KachakaError, Pose, PowerSupplyStatus};
use image::DynamicImage;
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

impl From<kachaka_api::RosImage> for DynamicImage {
    fn from(image: kachaka_api::RosImage) -> Self {
        match image.encoding.as_str() {
            "rgb8" => {
                let img_buffer = image::RgbImage::from_raw(image.width, image.height, image.data)
                    .expect("Failed to create image buffer");
                DynamicImage::ImageRgb8(img_buffer)
            }
            "rgba8" => {
                let img_buffer = image::RgbaImage::from_raw(image.width, image.height, image.data)
                    .expect("Failed to create image buffer");
                DynamicImage::ImageRgba8(img_buffer)
            }
            "bgr8" => {
                let mut rgb_data = image.data.clone();
                for pixel in rgb_data.chunks_mut(3) {
                    pixel.swap(0, 2);
                }
                let img_buffer = image::RgbImage::from_raw(image.width, image.height, rgb_data)
                    .expect("Failed to create image buffer");
                DynamicImage::ImageRgb8(img_buffer)
            }
            _ => panic!("Unsupported image encoding: {}", image.encoding),
        }
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
