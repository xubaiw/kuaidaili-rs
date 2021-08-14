// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod utils;

// account
mod get_account_balance;
pub use get_account_balance::*;

// order
mod get_order_expire_time;
pub use get_order_expire_time::*;

// dps
mod check_dps_valid;
mod get_dps;
mod get_dps_valid_time;
mod get_ip_balance;

// tps
mod tps_current_ip;
mod change_tps_ip;
mod get_tps;

// kps
mod get_kps;

// ops
mod get_ops;
mod check_ops_valid;

// general
mod get_ua;
mod get_ip_whitelist;
mod get_proxy_authorization;
mod set_ip_whitelist;
mod get_area_code;