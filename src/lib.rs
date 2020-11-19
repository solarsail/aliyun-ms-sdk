#![warn(clippy::all)]
mod action;
pub mod error;
mod param;
mod request;
mod response;
mod sign;

pub use action::*;
pub use param::*;
pub use request::*;
pub use response::*;

type ParamVec<'a> = Vec<(&'a str, &'a str)>;

#[cfg(test)]
mod tests {
    use super::*;
    const KEY: &str = "access_key";
    const SECRET: &str = "secret";

    #[test]
    fn send_sms() {
        let sms_param = SendSmsParam::new("13812345678".into(), "signature", "template_id")
            .template_param("user", "Azure".into())
            .out_id("123456".into());
        let action = SmsAction::SendSms(sms_param);
        let req = Request::new(KEY, SECRET, ApiModule::Sms(action));
        let resp = tokio_test::block_on(req.send());
        println!("{:?}", &resp);
        let resp = resp.unwrap().send_sms_response().unwrap();
        assert_eq!(resp.code.as_str(), "OK");
    }

    #[test]
    fn call_by_tts() {
        let voice_param = VoiceParam::new("051012345678", "13812345678".into(), "template_id")
            .template_param("name", "Alice".into())
            .speed(100)
            .out_id("123456".into());
        let action = VoiceAction::SingleCallByTts(voice_param);
        let req = Request::new(KEY, SECRET, ApiModule::Voice(action));
        let resp = tokio_test::block_on(req.send());
        println!("{:?}", &resp);
        let resp = resp.unwrap().single_call_by_tts_response().unwrap();
        assert_eq!(resp.code.as_str(), "OK");
    }
}
