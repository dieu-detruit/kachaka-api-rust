#[derive(Debug)]
pub struct KachakaError {
    pub error_code: i32,
}

#[derive(Debug)]
pub enum KachakaApiError {
    CommunicationError(tonic::Status),
    ApiError(KachakaError),
    NullResult,
}
