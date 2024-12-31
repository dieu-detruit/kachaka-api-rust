use crate::kachaka_api;

#[derive(Debug, Default)]
pub struct StartCommandOptions {
    pub title: String,
    pub tts_on_success: String,
    pub cancel_all: bool,
    pub deferrable: bool,
    pub lock_on_end: Option<kachaka_api::LockOnEnd>,
}

impl StartCommandOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn tts_on_success(mut self, tts_on_success: &str) -> Self {
        self.tts_on_success = tts_on_success.to_string();
        self
    }

    pub fn cancel_all(mut self, cancel_all: bool) -> Self {
        self.cancel_all = cancel_all;
        self
    }

    pub fn deferrable(mut self, deferrable: bool) -> Self {
        self.deferrable = deferrable;
        self
    }

    pub fn lock_on_end(mut self, lock_on_end: Option<kachaka_api::LockOnEnd>) -> Self {
        self.lock_on_end = lock_on_end;
        self
    }
}
