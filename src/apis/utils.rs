// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::Serialize;
use thiserror::Error;
use typenum::{N101, N102, N103, N104, N105, N106, N109, N110, N111};

use self::date_format::serialize;

/// Endpoint struct for different APIs 每个API使用不同的端点
pub struct Endpoint(&'static str);

impl Endpoint {
    /// Create a new endpoint constant
    ///
    /// 新建一个端点
    pub const fn new(s: &'static str) -> Self {
        Endpoint(s)
    }
}

impl ToString for Endpoint {
    /// the string of the endpoint
    ///
    /// 端点对应的字符串
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/// General error across different apis
///
/// 公共错误码
///
/// API: https://www.kuaidaili.com/doc/api/return/#errcode
#[derive(Debug, Error)]
pub enum GeneralError {
    /// Missing orderid parameter in request
    ///
    /// 缺少订单参数
    ///
    /// EC: -101
    #[error("orderid missing 缺少订单参数")]
    OrderIdMissing { code: N101, msg: String },
    /// Order specified by the orderid is invalid or expired
    ///
    /// 订单无效或已过期
    ///
    /// EC: -102
    #[error("order is invalid or expired 订单无效或已过期, code: {code:?}, msg: {msg}")]
    InvalidOrExpiredOrder { code: N102, msg: String },
    /// Missing signature parameter in request
    ///
    /// 缺少签名参数
    ///
    /// EC: -103
    #[error("signature missing 缺少签名参数, code: {code:?}, msg: {msg}")]
    SignatureMissing { code: N103, msg: String },
    /// Signature is wrong
    ///
    /// 签名错误
    ///
    /// EC: -104
    #[error("wrong signature 签名错误, code: {code:?}, msg: {msg}")]
    WrongSignature { code: N104, msg: String },
    /// Signature method is unknown
    ///
    /// 未知的签名方式
    ///
    /// EC: -105
    #[error("unknown signature method 未知的签名方式, code: {code:?}, msg: {msg}")]
    UnknownSignature { code: N105, msg: String },
    /// Signature is expired
    ///
    /// 签名已过期
    ///
    /// EC: -106
    #[error("expired signature 签名已过期, code: {code:?}, msg: {msg}")]
    ExpiredSignature { code: N106, msg: String },
    /// There are anomalies in kuaidaili system
    ///
    /// 系统异常
    ///
    /// EC: -109
    #[error("system anomaly 系统异常, code: {code:?}, msg: {msg}")]
    SystemAnomaly { code: N109, msg: String },
    /// Missing timestamp parameter in request
    ///
    /// 缺少时间戳参数
    ///
    /// EC: -110
    #[error("timestamp missing 缺少时间戳参数, code: {code:?}, msg: {msg}")]
    TimestampMissing { code: N110, msg: String },
    /// Timestamp is wrong
    ///
    /// 时间戳格式错误
    ///
    /// EC: -111
    #[error("wrong timestamp 时间戳格式错误, code: {code:?}, msg: {msg}")]
    WrongTimestamp { code: N111, msg: String },
}

/// Convertion between kuaidaili time string (which is UTC/GMT+08:00) and UTC time
///
/// 快代理时间字符串和UTC时间转换
pub mod date_format {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    /// serialize UTC time to kuaidaili time string
    ///
    /// UTC时间序列化为快代理时间字符串
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let beijing_offset = FixedOffset::east(5 * 3600);
        let date: DateTime<FixedOffset> =
            date.with_timezone(&TimeZone::from_offset(&beijing_offset));
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    /// deserialize kuaidaili time string to UTC time
    ///
    /// 快代理时间字符串反序列化为UTC时间
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let beijing_offset = FixedOffset::east(5 * 3600);
        beijing_offset
            .datetime_from_str(&s, FORMAT)
            .map(|d| d.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

/// Type of the proxy
///
/// 代理类型
#[derive(Clone, Debug)]
pub enum ProxyType {
    Http,
    Socks,
}

impl Serialize for ProxyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Http => serializer.serialize_i8(1),
            Self::Socks => serializer.serialize_i8(2),
        }
    }
}

/// carrier of proxy
///
/// 代理运营商
#[derive(Clone, Debug)]
pub enum ProxyCarrier {
    联通,
    电信,
}

impl Serialize for ProxyCarrier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::联通 => serializer.serialize_i8(1),
            Self::电信 => serializer.serialize_i8(2),
        }
    }
}

/// Convertion between bool and i8 (true = 1, false = 0)
///
/// 布尔值和数字转换
pub mod bool_int {

    use serde::{self, de::Visitor, Deserializer, Serializer};

    /// serialize bool to int
    ///
    /// 布尔值转换为数字
    pub fn serialize<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *b {
            serializer.serialize_i8(1)
        } else {
            serializer.serialize_i8(0)
        }
    }

    /// deserialize int to bool
    ///
    /// 数字转换为布尔值
    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoolVisitor;

        impl<'de> Visitor<'de> for BoolVisitor {
            type Value = bool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("0 or 1 expected")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(false),
                    1 => Ok(true),
                    _ => Err(serde::de::Error::custom(format!("expect 0 or 1"))),
                }
            }
        }

        Ok(deserializer.deserialize_i8(BoolVisitor)?)
    }
}
