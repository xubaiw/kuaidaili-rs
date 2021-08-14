// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::apis::utils::{Endpoint, GeneralError, ProxyCarrier, ProxyType};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typenum::{N1, N11, N12, N13, N14, N15, N16, N2, N3, N4, N5, N51, P2, P3, P4};

pub const ENDPOINT_GET_DPS: Endpoint = Endpoint::new("https://dps.kdlapi.com/api/getdps");

#[derive(Builder, Debug, Serialize)]
struct GetDpsArgs {
    num: i32,
    #[builder(default)]
    area: Option<Vec<String>>,
    #[builder(default)]
    area_ex: Option<Vec<String>>,
    #[builder(default)]
    ipstart: Option<Vec<String>>,
    #[builder(default)]
    ipstart_ex: Option<Vec<String>>,
    #[builder(default)]
    pt: Option<ProxyType>,
    #[builder(default = "false")]
    #[serde(with = "crate::apis::utils::bool_int")]
    st: bool,
    #[builder(default = "false")]
    f_loc: bool,
    #[builder(default = "false")]
    f_citycode: bool,
    #[builder(default = "false")]
    f_et: bool,
    #[builder(default = "false")]
    dedup: bool,
    #[builder(default)]
    carrier: Option<ProxyCarrier>,
}

#[derive(Debug, Deserialize)]
pub struct GetDpsData {
    count: i32,
    proxy_list: Vec<String>,
    dedup_count: Option<i32>,
    order_left_count: Option<i32>,
    order_left_today: Option<i32>,
}

#[derive(Debug, Error)]
pub enum GetDpsError {
    /// The order is expired
    ///
    /// 订单已过期
    #[error("expired order 订单已过期, code: {code:?}, msg: {msg}")]
    ExpiredOrder { code: P2, msg: String },
    /// No proxy available
    ///
    /// 暂无可用代理
    #[error("no proxy available 暂无可用代理, code: {code:?}, msg: {msg}")]
    NoProxyAvailable { code: P3, msg: String },
    /// The account is not authenticated
    ///
    /// 账号尚未通过实名认证
    #[error("account not authenticated 账号尚未通过实名认证, code: {code:?}, msg: {msg}")]
    AccountNotAuthenticated { code: P4, msg: String },
    /// Invalid Request
    ///
    /// 无效请求
    #[error("invalid request 无效请求, code: {code:?}, msg: {msg}")]
    InvalidRequest { code: N1, msg: String },
    /// Invalid Order
    ///
    /// 订单无效
    #[error("invalid order 订单无效, code: {code:?}, msg: {msg}")]
    InvalidOrder { code: N2, msg: String },
    /// Wrong argument
    ///
    /// 参数错误
    #[error("wrong arguemnt 参数错误, code: {code:?}, msg: {msg}")]
    WrongArgument { code: N3, msg: String },
    /// Extraction Failure
    ///
    /// 提取失败
    #[error("extraction failure 提取失败, code: {code:?}, msg: {msg}")]
    ExtractionFailure { code: N4, msg: String },
    /// Order is not dps
    ///
    /// 提取失败
    #[error("order is not dps 此订单不能提取私密代理, code: {code:?}, msg: {msg}")]
    OrderNotDps { code: N5, msg: String },
    /// Frequency limit
    ///
    /// 调用频率限制
    #[error("frequency limit 调用频率限制, code: {code:?}, msg: {msg}")]
    FrequencyLimit { code: N51, msg: String },
    /// Order is withdrawn
    ///
    /// 订单已退款
    #[error("order withdrawn 订单已退款, code: {code:?}, msg: {msg}")]
    OrderWithdrawn { code: N16, msg: String },
    /// Order is expired
    ///
    /// 订单已过期
    #[error("order expired 订单已过期, code: {code:?}, msg: {msg}")]
    OrderExpired { code: N15, msg: String },
    /// Order is blocked
    ///
    /// 订单被封禁
    #[error("order blocked 订单被封禁, code: {code:?}, msg: {msg}")]
    OrderBlocked { code: N14, msg: String },
    /// Transparent general error
    ///
    /// 通用错误
    #[error("general error 通用错误, source: {source:?}")]
    GeneralError {
        #[from]
        source: GeneralError,
    },
}
