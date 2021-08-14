// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::apis::utils::{Endpoint, GeneralError};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;
use typenum::N1;

pub const ENDPOINT_GET_ORDER_EXPIRE_TIME: Endpoint =
    Endpoint::new("https://dev.kdlapi.com/api/getorderexpiretime");

#[derive(Debug, Deserialize)]
pub struct GetOrderExpireTimeData {
    #[serde(with = "crate::apis::utils::date_format")]
    expire_time: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum GetAccountBalanceError {
    #[error("argument missing 缺少参数, code: {code:?}, msg: {msg}")]
    ArgumentMissing { code: N1, msg: String },
    #[error("general error 通用错误, source: {source:?}")]
    GeneralError {
        #[from]
        source: GeneralError,
    },
}
