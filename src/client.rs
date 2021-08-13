// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::auth::Auth;
use crate::errors::KuaidailiError;
use crate::EndPoint;
use std::collections::HashMap;

pub struct Client {
    auth: Auth,
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth: Auth) -> Client {
        Client { auth, client: reqwest::Client::new() }
    }

    /// Get the balance of your account
    /// 获取账户余额
    ///
    /// API: https://www.kuaidaili.com/doc/api/getaccountbalance/
    ///
    /// * `order_id` - the id of your proxy order - 代理订单id
    pub async fn get_account_balance(&self, order_id: String) -> Result<(), KuaidailiError> {
        match reqwest::get(format!(
            "https://dev.kdlapi.com/api/getaccountbalance?orderid={}&signature={}",
            order_id,
            self.auth.get_api_key()
        ))
        .await
        {
            Err(_) => Err(KuaidailiError::NetworkError),
            Ok(_response) => Ok(()),
        }
    }

    async fn send_request(
        &self,
        method: Method,
        endpoint: EndPoint,
        params: &HashMap<String, String>,
    ) -> Result<(), KuaidailiError> {
        let url = format!(
            "https://{}?{}",
            endpoint.to_string(),
            params
                .iter()
                .map(|(k, v)| { format!("{}={}", k, v) })
                .fold(String::new(), |a, b| format!("{}&{}", a, b))
        );
        let res = match method {
            Method::Get => self.client.get(url).send().await,
            Method::Post => self.client.post(url).send().await,
        };
        match res {
            Err(_) => Err(KuaidailiError::NetworkError),
            Ok(v) => {
                
                Ok(())
            }
        }
    }
}

enum Method {
    Get,
    Post
}
