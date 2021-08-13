// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT


pub struct Auth {
    order_id: String,
    api_key: String,
}

impl Auth {
    fn new(order_id: &str, api_key: &str) -> Self {
        Auth {
            order_id: order_id.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub fn get_order_id(&self) -> String {
        self.order_id.clone()
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}


