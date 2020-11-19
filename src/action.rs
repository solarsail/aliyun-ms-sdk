use crate::param::*;
use crate::ParamVec;

pub trait Action {
    fn name(&self) -> &'static str;
    fn method(&self) -> &'static str;
    fn params(&self) -> ParamVec;
}
pub enum SmsAction {
    SendSms(SendSmsParam),
}

impl Action for SmsAction {
    fn name(&self) -> &'static str {
        match self {
            SmsAction::SendSms(_) => "SendSms",
        }
    }

    fn method(&self) -> &'static str {
        "GET"
    }

    fn params(&self) -> ParamVec {
        match self {
            SmsAction::SendSms(p) => p.params(),
        }
    }
}

pub enum VoiceAction {
    SingleCallByTts(VoiceParam),
}

impl Action for VoiceAction {
    fn name(&self) -> &'static str {
        match self {
            VoiceAction::SingleCallByTts(_) => "SingleCallByTts",
        }
    }

    fn method(&self) -> &'static str {
        match self {
            VoiceAction::SingleCallByTts(_) => "GET",
        }
    }

    fn params(&self) -> ParamVec {
        match self {
            VoiceAction::SingleCallByTts(p) => p.params(),
        }
    }
}
