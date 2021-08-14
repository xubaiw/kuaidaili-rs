// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT


use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Client {
    order_id: String,
    api_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(order_id: String, api_key: String) -> Client {
        Client {
            order_id,
            api_key,
            client: reqwest::Client::new(),
        }
    }
    
    // async fn send_request<T>(
    //     &self,
    //     method: Method,
    //     endpoint: EndPoint,
    //     params: &HashMap<String, String>,
    // ) -> Result<KuaidailiResponse<T>, KuaidailiError>
    // where
    //     T: serde::de::DeserializeOwned,
    // {
    //     // make url using simple authorization
    //     let url = format!(
    //         "{endpoint}?{query}",
    //         endpoint = endpoint.to_string(),
    //         query = params
    //             .iter()
    //             .map(|(k, v)| { format!("{}={}", k, v) })
    //             .fold(String::new(), |a, b| format!("{}&{}", a, b))
    //     );

    //     // send
    //     let res = match method {
    //         Method::Get => self.client.get(url).send().await,
    //         Method::Post => self.client.post(url).send().await,
    //     };
    //     if res.is_err() {
    //         return Err(KuaidailiError::NetworkError {
    //             source: res.unwrap_err(),
    //         });
    //     }
    //     let res = res.unwrap().json::<KuaidailiResponse<T>>().await;
    //     if res.is_err() {
    //         return Err(KuaidailiError::DeserializeError);
    //     }
    //     Ok(res.unwrap())
    // }
}

/// 快代理返回结果
#[derive(Debug, Deserialize)]
struct KuaidailiResponse<T> {
    code: i32,
    data: T,
    msg: String,
}

#[derive(Builder, Debug, Serialize)]
struct KuaidailiRequest<T> {
    #[serde(flatten)]
    args: T,
    #[serde(rename = "orderid")]
    order_id: String,
}


// fn deserialize_i32_from_string<'de, D>(d: D) -> Result<i32, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     struct FromStringVisitor;
//     impl<'de> serde::de::Visitor<'de> for FromStringVisitor {
//         type Value = i32;

//         fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//             formatter.write_str("a string of number")
//         }

//         fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//         where
//             E: serde::de::Error,
//         {
//             match v.parse::<i32>() {
//                 Ok(v) => Ok(v),
//                 Err(e) => Err(E::custom(format!("cannot parse into i32: {:?}", e))),
//             }
//         }

//         fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//         where
//             E: serde::de::Error,
//         {
//             self.visit_str(&v)
//         }
//     }

//     let visitor = FromStringVisitor;
//     d.deserialize_string(visitor)
// }

// enum Method {
//     Get,
//     Post,
// }
