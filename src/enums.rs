// Copyright (c) 2021 Xubai Wang
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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

pub struct EndPoint(&'static str);

impl ToString for EndPoint {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub const GET_ORDER_EXPIRE_TIME: EndPoint = EndPoint("dev.kdlapi.com/api/getorderexpiretime");
pub const GET_IP_WHITELIST: EndPoint = EndPoint("dev.kdlapi.com/api/getipwhitelist");
pub const SET_IP_WHITELIST: EndPoint = EndPoint("dev.kdlapi.com/api/setipwhitelist");
pub const GET_KPS_PROXY: EndPoint = EndPoint("kps.kdlapi.com/api/getkps");
pub const GET_DPS_PROXY: EndPoint = EndPoint("dps.kdlapi.com/api/getdps");
pub const GET_OPS_PROXY_NORMAL_OR_VIP: EndPoint = EndPoint("dev.kdlapi.com/api/getproxy");
pub const GET_OPS_PROXY_SVIP: EndPoint = EndPoint("svip.kdlapi.com/api/getproxy");
pub const GET_OPS_PROXY_ENT: EndPoint = EndPoint("ent.kdlapi.com/api/getproxy");
pub const CHECK_DPS_VALID: EndPoint = EndPoint("dps.kdlapi.com/api/checkdpsvalid");
pub const CHECK_OPS_VALID: EndPoint = EndPoint("dev.kdlapi.com/api/checkopsvalid");
pub const GET_IP_BALANCE: EndPoint = EndPoint("dps.kdlapi.com/api/getipbalance");
pub const GET_DPS_VALID_TIME: EndPoint = EndPoint("dps.kdlapi.com/api/getdpsvalidtime");
pub const TPS_CURRENT_IP: EndPoint = EndPoint("tps.kdlapi.com/api/tpscurrentip");
pub const CHANGE_TPS_IP: EndPoint = EndPoint("tps.kdlapi.com/api/changetpsip");
pub const GET_TPS: EndPoint = EndPoint("tps.kdlapi.com/api/gettps");
pub const GET_PROXY_AUTHORIZATION: EndPoint = EndPoint("dev.kdlapi.com/api/getproxyauthorization");
pub const GET_UA: EndPoint = EndPoint("www.kuaidaili.com/api/getua");
pub const GET_AREA_CODE: EndPoint = EndPoint("dev.kdlapi.com/api/getareacode");
pub const GET_ACCOUNT_BALANCE: EndPoint = EndPoint("dev.kdlapi.com/api/getaccountbalance");
