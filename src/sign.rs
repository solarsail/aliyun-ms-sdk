use std::collections::BTreeMap;
use std::iter::FromIterator;

use chrono::prelude::*;
use hmac::{Hmac, Mac, NewMac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn make_signature<'a>(
    method: &str,
    apikey: &str,
    secret: &str,
    params: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> (String, String) {
    const DEFAULT_PARAMS: &[(&str, &str)] = &[
        ("SignatureMethod", "HMAC-SHA1"),
        ("SignatureVersion", "1.0"),
        ("Format", "json"),
        ("RegionId", "cn-hangzhou"),
    ];
    let mut param_map = BTreeMap::from_iter(params.into_iter());
    param_map.extend(DEFAULT_PARAMS.iter().map(|(k, v)| (*k, *v)));
    let now = Utc::now();
    let nonce = format!("{}", now.timestamp_subsec_nanos());
    let ts = now.to_string();
    param_map.insert("SignatureNonce", &nonce);
    param_map.insert("Timestamp", &ts);
    param_map.insert("AccessKeyId", apikey);
    param_map.remove("Signature");
    let mut sorted_params_str = param_map.iter().fold(String::new(), |mut s, (k, v)| {
        s.push_str(&special_url_encode(k));
        s.push_str("=");
        s.push_str(&special_url_encode(&v));
        s.push_str("&");
        s
    });
    sorted_params_str.pop();
    let sign_str = format!("{}&%2F&{}", method, special_url_encode(&sorted_params_str));
    let sign_key = format!("{}&", secret);
    (sign(sign_key, sign_str), sorted_params_str)
}

fn sign(key: String, s: String) -> String {
    let mut mac = HmacSha1::new_varkey(key.as_bytes()).unwrap();
    mac.update(s.as_bytes());
    let result = mac.finalize().into_bytes();
    let sign = base64::encode(result);
    special_url_encode(sign)
}

fn special_url_encode<S: AsRef<str>>(s: S) -> String {
    let s: String = form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect();
    s.replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}
