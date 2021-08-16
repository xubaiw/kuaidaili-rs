// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/// A struct that holds the order_id, api_key, which can calculate the signature.
///
/// 用于保存用户orderid、apiKey，以及计算签名的对象。
///
/// # Example
/// ```
/// use kuaidaili::{Auth, Client};
/// let auth = Auth::new("your_order_id", "your_api_key");
/// let client = Client::new(auth);
/// ```
pub struct Auth {
    order_id: String,
    api_key: String,
}

impl Auth {
    /// Create a new [`Auth`] struct
    ///
    /// 新建一个[`Auth`]结构体
    pub fn new(order_id: &str, api_key: &str) -> Self {
        Self {
            order_id: order_id.into(),
            api_key: api_key.into(),
        }
    }

    pub fn get_order_id(&self) -> String {
        self.order_id.to_string()
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.to_string()
    }

    // fn get_string_to_sign(
    //     method: &str,
    //     endpoint: Endpoint,
    //     params: Vec<(String, String)>,
    // ) -> String {
    //     let mut params = params;
    //     params.sort();
    //     format!(
    //         "{method}{endpoint}?{query_string}",
    //         method = method,
    //         endpoint = endpoint.first_part(),
    //         query_string = params
    //             .into_iter()
    //             .map(|(k, v)| format!("{}={}", k, v))
    //             .collect::<Vec<_>>()
    //             .join("&"),
    //     )
    // }

    // pub fn sign_str(&self, raw_str: &str, method: String) {
    //     self.
    // }
}
