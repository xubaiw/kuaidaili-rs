// Copyright (c) 2021 Xubai Wang
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KuaidailiError {
    /// Represents a failure due to network
    #[error("Http operation fails")]
    NetworkError
}