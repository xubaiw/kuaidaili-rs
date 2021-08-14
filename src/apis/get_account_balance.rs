// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::apis::utils::{Endpoint, GeneralError};
use serde::Deserialize;
use thiserror::Error;
use typenum::{N1, N10, P2, P3};

/// Endpoint for  GetAccountBalance API
///
/// 获取账号余额接口
///
/// API: https://www.kuaidaili.com/doc/api/getaccountbalance/#1
pub const ENDPOINT_GET_ACCOUNT_BALANCE: Endpoint =
    Endpoint::new("https://dev.kdlapi.com/api/getaccountbalance");

/// Data for GetAccountBalance API
///
/// 获取账号余额返回值
///
/// API: https://www.kuaidaili.com/doc/api/getaccountbalance/#4
#[derive(Debug, Deserialize)]
pub struct GetAccountBalanceData {
    balance: i32,
}

/// Error for GetAccountBalance API
///
/// 获取账号余额错误类型
///
/// API: https://www.kuaidaili.com/doc/api/getaccountbalance/#3
#[derive(Debug, Error)]
pub enum GetAccountBalanceError {
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
    /// Missing arguments
    ///
    /// 缺少参数
    #[error("argument missing 缺少参数, code: {code:?}, msg: {msg}")]
    ArgumentMissing { code: N1, msg: String },
    /// Reach the call frequency limit
    ///
    /// 接口调用超出频率
    #[error("frequency limit 接口调用超出频率, code: {code:?}, msg: {msg}")]
    FrequencyLimit { code: N10, msg: String },
    /// Transparent general error
    ///
    /// 通用错误
    #[error("general error 通用错误, source: {source:?}")]
    GeneralError {
        #[from]
        source: GeneralError,
    },
}
