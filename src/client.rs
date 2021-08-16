// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use crate::{auth::Auth, endpoint::*, errors::KdlError};
use derive_builder::Builder;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

pub struct Client {
    auth: Auth,
}

impl Client {
    pub fn new(auth: Auth) -> Self {
        Self { auth }
    }

    /// Get the expire time of the order (order_id specified in auth)
    ///
    /// 获取订单到期时间，订单在auth中指定
    pub async fn get_order_expire_time(&self) -> KdlResult<GetOrderExpireTimeRets> {
        Ok(self
            .make_request(GET_ORDER_EXPIRE_TIME, GetOrderExpireTimeArgs)
            .await?)
    }

    /// Get authorization info of the order
    ///
    /// 获取指定订单访问代理IP的鉴权信息。鉴权信息包含用户名密码，用于请求私密代理/独享代理/隧道代理时进行身份验证。
    pub async fn get_proxy_authorization(&self) -> KdlResult<GetProxyAuthorizationRets> {
        Ok(self
            .make_request(
                GET_PROXY_AUTHORIZATION,
                GetProxyAuthorizationArgsBuilder::default()
                    .build()
                    .map_err(|_| KdlError::BuidlerError)?,
            )
            .await?)
    }

    /// get balance of the account
    ///
    /// 获取账户余额
    pub async fn get_account_balance(&self) -> KdlResult<GetAccountBalanceRets> {
        Ok(self
            .make_request(GET_ACCOUNT_BALANCE, GetAccountBalanceArgs)
            .await?)
    }

    /// Get User-Agent
    ///
    /// 获取User-Agent
    pub async fn get_ua(&self, args: GetUAArgs) -> KdlResult<GetUARets> {
        Ok(self.make_request(GET_UA, args).await?)
    }

    /// Get the code of an area
    ///
    /// 获取指定地区编码
    pub async fn get_area_code(&self, args: GetAreaCodeArgs) -> KdlResult<GetAreaCodeRets> {
        Ok(self.make_request(GET_AREA_CODE, args).await?)
    }

    pub async fn check_ops_valid(&self, args: CheckOpsValidArgs) -> KdlResult<CheckOpsValidRets> {
        Ok(self.make_request(CHECK_OPS_VALID, args).await?)
    }

    /// Get ip balance
    ///
    /// 获取订单ip余额，此接口只对按量付费订单和包年包月的集中提取型订单有效
    pub async fn get_ip_balance(&self) -> KdlResult<GetIpBalanceRets> {
        Ok(self.make_request(GET_IP_BALANCE, GetIpBalanceArgs).await?)
    }

    /// Get ip whitelist
    ///
    /// 获取ip白名单
    pub async fn get_ip_whitelist(&self) -> KdlResult<GetIpWhitelistRets> {
        Ok(self
            .make_request(GET_IP_WHITELIST, GetIpWhitelistArgs)
            .await?)
    }

    /// Check if dps is valid
    ///
    /// 检测私密代理有效性
    pub async fn check_dps_valid(&self, args: CheckDpsValidArgs) -> KdlResult<CheckDpsValidRets> {
        Ok(self.make_request(CHECK_DPS_VALID, args).await?)
    }

    /// set ip whitelist
    ///
    /// 设置订单的ip白名单
    pub async fn set_ip_whitelist(
        &self,
        args: SetIpWhitelistArgs,
    ) -> KdlResult<SetIpWhitelistRets> {
        Ok(self.make_request(SET_IP_WHITELIST, args).await?)
    }

    /// Get tps current ip
    ///
    /// 获取隧道当前的IP，仅支持支持换IP周期>=1分钟的隧道代理订单
    pub async fn tps_current_ip(&self) -> KdlResult<TpsCurrentIpRets> {
        Ok(self.make_request(TPS_CURRENT_IP, TpsCurrentIpArgs).await?)
    }

    /// change tps ip
    ///
    /// 更换tps当前ip
    pub async fn change_tps_ip(&self) -> KdlResult<ChangeTpsIpRets> {
        Ok(self.make_request(CHANGE_TPS_IP, ChangeTpsIpArgs).await?)
    }

    /// Get tps
    ///
    /// 获取隧道代理IP
    pub async fn get_tps(&self, args: GetTpsArgs) -> KdlResult<GetTpsRets> {
        Ok(self.make_request(GET_TPS, args).await?)
    }

    /// get dps valid time
    ///
    /// 获取私密代理ip有效时间
    pub async fn get_dps_valid_time(
        &self,
        args: GetDpsValidTimeArgs,
    ) -> KdlResult<GetDpsValidTimeRets> {
        Ok(self.make_request(GET_DPS_VALID_TIME, args).await?)
    }

    /// get dps
    ///
    /// 获取私密代理
    pub async fn get_dps(&self, args: GetDpsArgs) -> KdlResult<GetDpsRets> {
        Ok(self.make_request(GET_DPS_PROXY, args).await?)
    }

    /// get kps
    /// 
    /// 获取独享代理
    pub async fn get_kps(&self, args: GetKpsArgs) -> KdlResult<GetKpsRets> {
        Ok(self.make_request(GET_KPS_PROXY, args).await?)
    }

    /// get proxy
    ///
    /// 获取开放代理
    pub async fn get_proxy(&self, args: GetProxyArgs) -> KdlResult<GetProxyRets> {
        Ok(self.make_request(GET_OPS_PROXY_NORMAL_OR_VIP, args).await?)
    }

    /// Internal basic request method
    ///
    /// 基本请求
    async fn make_request<A, R>(&self, endpoint: Endpoint, args: A) -> Result<R, KdlError>
    where
        A: Serialize,
        R: DeserializeOwned,
    {
        let url = format!(
            "https://{endpoint}?orderid={order_id}&signature={api_key}&{query_string}",
            endpoint = endpoint.to_string(),
            order_id = self.auth.get_order_id(),
            api_key = self.auth.get_api_key(),
            query_string = serde_qs::to_string(&args)?
        );
        let res = reqwest::get(url).await?.text().await?;
        let res = serde_json::from_str::<KdlRet<R>>(&res)?;
        match res.code {
            0 => Ok(res.data),
            c => Err(KdlError::CodeError {
                code: c,
                message: res.message,
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct KdlRet<Data> {
    #[serde(rename = "msg")]
    message: String,
    code: i32,
    data: Data,
}

pub type KdlResult<T> = Result<T, KdlError>;

/// args of set_ip_whitelist
#[derive(Builder, Debug, Serialize)]
pub struct SetIpWhitelistArgs {
    #[serde(rename = "iplist")]
    ip_list: String,
}

/// return values if set_ip_whitelist
pub type SetIpWhitelistRets = String;

/// args of tps_current_ip
#[derive(Debug, Serialize)]
pub struct TpsCurrentIpArgs;

/// return values if tps_current_ip
#[derive(Debug, Deserialize)]
pub struct TpsCurrentIpRets {
    current_ip: String,
}

/// args of change_tps_ip
#[derive(Debug, Serialize)]
pub struct ChangeTpsIpArgs;

/// return values of change_tps_ip
#[derive(Debug, Deserialize)]
pub struct ChangeTpsIpRets {
    current_ip: String,
}

/// args of get_tps
#[derive(Builder, Debug, Serialize)]
pub struct GetTpsArgs {
    #[builder(default = "1")]
    num: u32,
    #[builder(default = "1")]
    pt: u32,
    #[builder(default = "\"json\".to_string()")]
    format: String,
    #[builder(default = "\"\n\".to_string()")]
    sep: String,
}

/// return values of get_tps
#[derive(Debug, Deserialize)]
pub struct GetTpsRets {
    count: u32,
    proxy_list: Vec<String>,
}

/// args of get_dps_valid_time
#[derive(Builder, Debug, Serialize)]
pub struct GetDpsValidTimeArgs {
    proxy: String,
}

/// return values of get_dps_valid_time
pub type GetDpsValidTimeRets = HashMap<String, u32>;

/// args of get_dps
#[derive(Builder, Debug, Serialize)]
pub struct GetDpsArgs {
    num: u32,
    #[builder(default = "\"json\".to_string()")]
    format: String,
    #[builder(default = "\"\n\".to_string()")]
    sep: String,
}

/// return values of get_tps
#[derive(Debug, Deserialize)]
pub struct GetDpsRets {
    count: u32,
    dedup_count: u32,
    order_left_count: u32,
    today_left_count: u32,
    proxy_list: Vec<String>,
}

/// args of get_kps
#[derive(Builder, Debug, Serialize)]
pub struct GetKpsArgs {
    num: u32,
    #[builder(default = "\"json\".to_string()")]
    format: String,
    #[builder(default = "\"\n\".to_string()")]
    sep: String,
}

/// return values of get_kps
#[derive(Debug, Deserialize)]
pub struct GetKpsRets {
    count: u32,
    proxy_list: Vec<String>,
}

/// args of get_proxy
#[derive(Builder, Debug, Serialize)]
pub struct GetProxyArgs {
    num: u32,
    #[builder(default = "\"json\".to_string()")]
    format: String,
    #[builder(default = "\"\n\".to_string()")]
    sep: String,
}

/// return values of get_proxy
#[derive(Debug, Deserialize)]
pub struct GetProxyRets {
    count: u32,
    proxy_list: Vec<String>,
}


//     def get_proxy(self, num=None, order_level=OpsOrderLevel.NORMAL, sign_type="simple", **kwargs):
//         """获取开放代理, 默认不需要鉴权
//            :param num: 提取数量, sign_type: 鉴权方式, order_level: 开放代理订单类型
//            :param kwargs: 其他关键字参数，具体有那些参数请查看帮助中心api说明
//            :return 若为json格式, 则返回data中proxy_list部分, 即proxy列表, 否则原样返回
//         """
//         if num is None:
//             raise KdlNameError("miss param: num")
//         if not isinstance(num, int):
//             KdlTypeError("num should be a integer")
//         endpoint = EndPoint.GetOpsProxyNormalOrVip.value
//         if order_level == OpsOrderLevel.SVIP:
//             endpoint = EndPoint.GetOpsProxySvip.value
//         if order_level == OpsOrderLevel.PRO:
//             endpoint = EndPoint.GetOpsProxyEnt.value

//         params = self._get_params(endpoint, num=num, sign_type=sign_type, **kwargs)
//         res = self._get_base_res("GET", endpoint, params)
//         if isinstance(res, dict):
//             return res['data']['proxy_list']
//         return res

/// args of check_dps_valid
#[derive(Debug, Serialize)]
pub struct CheckDpsValidArgs {
    proxy: String,
}

/// return values of check_dps_valid
pub type CheckDpsValidRets = HashMap<String, bool>;

/// args of get_ip_whitelist
#[derive(Debug, Serialize)]
pub struct GetIpWhitelistArgs;

/// return values of get_ip_whitelist
#[derive(Debug, Deserialize)]
pub struct GetIpWhitelistRets {
    count: u32,
    #[serde(rename = "ipwhitelist")]
    ip_whitelist: Vec<String>,
    limit: u32,
}

/// args of get_order_expire_time
#[derive(Debug, Serialize)]
pub struct GetOrderExpireTimeArgs;

/// return values of get_order_expire_time
#[derive(Debug, Deserialize)]
pub struct GetOrderExpireTimeRets {
    expire_time: String,
}

/// args of get_proxy_authorization
#[derive(Builder, Debug, Serialize)]
pub struct GetProxyAuthorizationArgs {
    #[builder(default = "1")]
    plain_text: u8,
}

/// return values of get_proxy_authorization
#[derive(Debug, Deserialize)]
pub struct GetProxyAuthorizationRets {
    r#type: String,
    credentials: String,
    username: String,
    password: String,
}

/// args of get_account_balance
#[derive(Debug, Serialize)]
pub struct GetAccountBalanceArgs;

/// return values of get_account_balance
#[derive(Debug, Deserialize)]
pub struct GetAccountBalanceRets {
    #[serde(deserialize_with = "deserialize_string_to_f32")]
    balance: f32,
}

/// args of get_ua
#[derive(Builder, Debug, Serialize)]
pub struct GetUAArgs {
    #[builder(default = "1")]
    num: u32,
    #[builder(default = "vec![UADeviceType::All]")]
    dt: Vec<UADeviceType>,
    #[builder(default = "vec![UAPlatform::All]")]
    platform: Vec<UAPlatform>,
    #[builder(default = "vec![UABrowser::All]")]
    browser: Vec<UABrowser>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UADeviceType {
    All,
    Pc,
    Mobile,
    Pad,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UAPlatform {
    All,
    Win,
    Macos,
    Linux,
    Ios,
    Android,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UABrowser {
    All,
    Chrome,
    Firefox,
    Ie,
    Weixin,
}

/// return values of get_ua
#[derive(Debug, Deserialize)]
pub struct GetUARets {
    count: u32,
    ua_list: Vec<String>,
}

/// args of get_area_code
#[derive(Builder, Debug, Serialize)]
pub struct GetAreaCodeArgs {
    area: String,
}

/// return values of get_area_code
#[derive(Debug, Deserialize)]
pub struct GetAreaCodeRets {
    #[serde(rename = "are_anme")]
    area_name: String,
    #[serde(rename = "are_code")]
    area_code: String,
}

// args of check_ops_valid
#[derive(Builder, Debug, Serialize)]
pub struct CheckOpsValidArgs {
    proxy: String,
}

/// return values of get_arecheck_ops_valida_code
pub type CheckOpsValidRets = HashMap<String, bool>;

/// args of get_ip_balance
#[derive(Debug, Serialize)]
pub struct GetIpBalanceArgs;

/// return values of get_area_code
#[derive(Debug, Deserialize)]
pub struct GetIpBalanceRets {
    balance: u32,
}

fn deserialize_string_to_f32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    s.parse::<f32>()
        .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))
}
