use std::cell::RefCell;
use std::collections::HashMap;

use crate::ParamVec;

pub struct SendSmsParam {
    phone: String,
    sign_name: &'static str,
    template: &'static str,
    template_param: HashMap<&'static str, String>,
    tp_str: RefCell<String>,
    extend: Option<String>,
    outid: Option<String>,
}

impl SendSmsParam {
    pub fn new(phone: String, sign_name: &'static str, template: &'static str) -> Self {
        Self {
            phone,
            sign_name,
            template,
            template_param: HashMap::new(),
            tp_str: RefCell::new(String::new()),
            extend: None,
            outid: None,
        }
    }

    pub fn template_param(self, k: &'static str, v: String) -> Self {
        let mut tps = self.template_param;
        tps.insert(k, v);
        Self {
            template_param: tps,
            ..self
        }
    }

    pub fn extend_code(self, extend: String) -> Self {
        Self {
            extend: Some(extend),
            ..self
        }
    }

    pub fn out_id(self, id: String) -> Self {
        Self {
            outid: Some(id),
            ..self
        }
    }

    pub fn params(&self) -> ParamVec {
        let mut params = vec![
            ("PhoneNumbers", self.phone.as_str()),
            ("SignName", self.sign_name),
            ("TemplateCode", self.template),
        ];
        if !self.template_param.is_empty() {
            // make JSON format template param string
            let tp = serde_json::to_string(&self.template_param).unwrap();
            self.tp_str.replace(tp);
            params.push(("TemplateParam", unsafe {
                // NOTE: the ParamVec returned should be consumed immediately
                //       to avoid probable dangling refs
                // SAFETY: the ParamVec is consumed immediately by make_signature(),
                //         see Request::send()
                self.tp_str.try_borrow_unguarded().unwrap().as_str()
            }));
        }
        if let Some(ref code) = self.extend {
            params.push(("SmsUpExtendCode", &code));
        }
        if let Some(ref id) = self.outid {
            params.push(("OutId", &id));
        }
        params
    }
}

pub struct VoiceParam {
    caller: &'static str,
    callee: String,
    template: &'static str,
    template_param: HashMap<&'static str, String>,
    tp_str: RefCell<String>,
    repeat: Option<String>,
    volume: Option<String>,
    speed: Option<String>,
    outid: Option<String>,
}

impl VoiceParam {
    pub fn new(caller: &'static str, callee: String, template: &'static str) -> Self {
        Self {
            caller,
            callee,
            template,
            template_param: HashMap::new(),
            tp_str: RefCell::new(String::new()),
            repeat: None,
            volume: None,
            speed: None,
            outid: None,
        }
    }

    pub fn template_param(self, k: &'static str, v: String) -> Self {
        let mut tps = self.template_param;
        tps.insert(k, v);
        Self {
            template_param: tps,
            ..self
        }
    }

    pub fn repeat(self, times: u8) -> Self {
        Self {
            repeat: Some(format!("{}", times)),
            ..self
        }
    }

    pub fn volume(self, v: u8) -> Self {
        Self {
            volume: Some(format!("{}", v)),
            ..self
        }
    }

    pub fn speed(self, s: u8) -> Self {
        Self {
            speed: Some(format!("{}", s)),
            ..self
        }
    }

    pub fn out_id(self, id: String) -> Self {
        Self {
            outid: Some(id),
            ..self
        }
    }

    pub fn params(&self) -> ParamVec {
        let mut params = vec![
            ("CalledNumber", self.callee.as_str()),
            ("CalledShowNumber", self.caller),
            ("TtsCode", self.template),
        ];
        if !self.template_param.is_empty() {
            // make JSON format template param string
            let tp = serde_json::to_string(&self.template_param).unwrap();
            self.tp_str.replace(tp);
            params.push(("TtsParam", unsafe {
                // NOTE: the ParamVec returned should be consumed immediately
                //       to avoid probable dangling refs
                // SAFETY: the ParamVec is consumed immediately by make_signature(),
                //         see Request::send()
                self.tp_str.try_borrow_unguarded().unwrap().as_str()
            }));
        }
        if let Some(ref times) = self.repeat {
            params.push(("PlayTimes", &times));
        }
        if let Some(ref v) = self.volume {
            params.push(("Volume", &v));
        }
        if let Some(ref s) = self.speed {
            params.push(("Speed", &s));
        }
        if let Some(ref id) = self.outid {
            params.push(("OutId", &id));
        }
        params
    }
}
