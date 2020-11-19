use serde::Deserialize;

use crate::error::IncorrectTypeError;

#[derive(Debug)]
pub struct Response {
    resp_str: String,
}

impl Response {
    pub fn send_sms_response(&self) -> Result<SendSmsResp, IncorrectTypeError> {
        serde_json::from_str(&self.resp_str).map_err(|_| IncorrectTypeError)
    }

    pub fn single_call_by_tts_response(&self) -> Result<SingleCallByTtsResp, IncorrectTypeError> {
        serde_json::from_str(&self.resp_str).map_err(|_| IncorrectTypeError)
    }
}

impl From<String> for Response {
    fn from(resp_str: String) -> Self {
        Self { resp_str }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendSmsResp {
    pub message: String,
    pub request_id: String,
    pub biz_id: Option<String>,
    pub code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SingleCallByTtsResp {
    pub message: String,
    pub request_id: String,
    pub call_id: Option<String>,
    pub code: String,
}
