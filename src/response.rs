use serde::Deserialize;

use crate::error::Error;

#[derive(Debug)]
pub struct Response {
    resp_str: String,
}

impl Response {
    pub fn send_sms_response(&self) -> Result<SendSmsResp, Error> {
        let r = serde_json::from_str::<SendSmsResp>(&self.resp_str)
            .map_err(|_| Error::IncorrectTypeError);
        r.and_then(|resp| {
            if resp.code == "OK" {
                Ok(resp)
            } else {
                let message = format!("{:?} {}", resp.biz_id, resp.message);
                Err(Error::ApiError {
                    code: resp.code,
                    request_id: resp.request_id,
                    message,
                })
            }
        })
    }

    pub fn single_call_by_tts_response(&self) -> Result<SingleCallByTtsResp, Error> {
        let r = serde_json::from_str::<SingleCallByTtsResp>(&self.resp_str)
            .map_err(|_| Error::IncorrectTypeError);
        r.and_then(|resp| {
            if resp.code == "OK" {
                Ok(resp)
            } else {
                let message = format!("{:?} {}", resp.call_id, resp.message);
                Err(Error::ApiError {
                    code: resp.code,
                    request_id: resp.request_id,
                    message,
                })
            }
        })
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
