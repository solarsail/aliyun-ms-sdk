use crate::{action::*, error::RequestError, response::Response, sign::make_signature, ParamVec};

pub enum ApiModule {
    Sms(SmsAction),
    Voice(VoiceAction),
}

impl ApiModule {
    pub fn domain(&self) -> &'static str {
        match self {
            ApiModule::Sms(_) => "dysmsapi.aliyuncs.com",
            ApiModule::Voice(_) => "dyvmsapi.aliyuncs.com",
        }
    }

    pub fn version(&self) -> &'static str {
        match self {
            ApiModule::Sms(_) => "2017-05-25",
            ApiModule::Voice(_) => "2017-05-25",
        }
    }

    pub fn params(&self) -> ParamVec {
        match self {
            ApiModule::Sms(p) => p.params(),
            ApiModule::Voice(p) => p.params(),
        }
    }

    pub fn action(&self) -> &'static str {
        match self {
            ApiModule::Sms(p) => p.name(),
            ApiModule::Voice(p) => p.name(),
        }
    }

    pub fn method(&self) -> &'static str {
        match self {
            ApiModule::Sms(p) => p.method(),
            ApiModule::Voice(p) => p.method(),
        }
    }
}

pub struct Request {
    module: ApiModule,
    key: &'static str,
    secret: &'static str,
}

impl Request {
    pub fn new(key: &'static str, secret: &'static str, module: ApiModule) -> Self {
        Self {
            key,
            secret,
            module,
        }
    }

    pub fn params(&self) -> ParamVec {
        let mut p = vec![
            ("Action", self.module.action()),
            ("Version", self.module.version()),
        ];
        let params = self.module.params();
        p.extend(params.into_iter());
        p
    }

    pub async fn send(&self) -> Result<Response, RequestError> {
        let (sign, params) =
            make_signature(self.module.method(), self.key, self.secret, self.params());
        #[cfg(test)]
        {
            println!("params: {}", &params);
        }
        let url = format!(
            "https://{}/?Signature={}&{}",
            self.module.domain(),
            sign,
            params
        );
        Ok(Response::from(reqwest::get(&url).await?.text().await?))
    }
}
