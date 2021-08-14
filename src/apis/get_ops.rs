// Copyright (c) 2021 Xubai Wang
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::apis::utils::Endpoint;

pub struct OpsOrderLevel(&'static str);

impl ToString for OpsOrderLevel {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
pub const NORMAL: OpsOrderLevel = OpsOrderLevel("dev");
pub const VIP: OpsOrderLevel = OpsOrderLevel("dev");
pub const SVIP: OpsOrderLevel = OpsOrderLevel("svip");
pub const PRO: OpsOrderLevel = OpsOrderLevel("ent");

pub const ENDPOINT_GET_OPS_PROXY_NORMAL_OR_VIP: Endpoint = Endpoint::new("https://dev.kdlapi.com/api/getproxy");
pub const ENDPOINT_GET_OPS_PROXY_SVIP: Endpoint = Endpoint::new("https://svip.kdlapi.com/api/getproxy");
pub const ENDPOINT_GET_OPS_PROXY_ENT: Endpoint = Endpoint::new("https://ent.kdlapi.com/api/getproxy");