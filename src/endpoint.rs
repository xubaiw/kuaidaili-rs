// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/// Url path for each API
///
/// 各个api的主机+路径
pub struct Endpoint {
    host_first: &'static str,
    path_last: &'static str,
}

impl Endpoint {
    const fn new(host_first: &'static str, path_last: &'static str) -> Self {
        Self {
            host_first,
            path_last,
        }
    }

    // pub fn get_path(&self) -> String {
    //     format!("/api/{}", self.path_last)
    // }
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        format!("{}.kdlapi.com/api/{}", self.host_first, self.path_last)
    }
}

pub const GET_ORDER_EXPIRE_TIME: Endpoint = Endpoint::new("dev", "getorderexpiretime");
pub const GET_IP_WHITELIST: Endpoint = Endpoint::new("dev", "getipwhitelist");
pub const SET_IP_WHITELIST: Endpoint = Endpoint::new("dev", "setipwhitelist");
pub const GET_KPS_PROXY: Endpoint = Endpoint::new("kps", "getkps");
pub const GET_DPS_PROXY: Endpoint = Endpoint::new("dps", "getdps");
pub const GET_OPS_PROXY_NORMAL_OR_VIP: Endpoint = Endpoint::new("dev", "getproxy");
// pub const GET_OPS_PROXY_SVIP: Endpoint = Endpoint::new("svip", "getproxy");
// pub const GET_OPS_PROXY_ENT: Endpoint = Endpoint::new("ent", "getproxy");
pub const CHECK_DPS_VALID: Endpoint = Endpoint::new("dps", "checkdpsvalid");
pub const CHECK_OPS_VALID: Endpoint = Endpoint::new("dev", "checkopsvalid");
pub const GET_IP_BALANCE: Endpoint = Endpoint::new("dps", "getipbalance");
pub const GET_DPS_VALID_TIME: Endpoint = Endpoint::new("dps", "getdpsvalidtime");
pub const TPS_CURRENT_IP: Endpoint = Endpoint::new("tps", "tpscurrentip");
pub const CHANGE_TPS_IP: Endpoint = Endpoint::new("tps", "changetpsip");
pub const GET_TPS: Endpoint = Endpoint::new("tps", "gettps");
pub const GET_PROXY_AUTHORIZATION: Endpoint = Endpoint::new("dev", "getproxyauthorization");
pub const GET_UA: Endpoint = Endpoint::new("www", "getua");
pub const GET_AREA_CODE: Endpoint = Endpoint::new("dev", "getareacode");
pub const GET_ACCOUNT_BALANCE: Endpoint = Endpoint::new("dev", "getaccountbalance");
