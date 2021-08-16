// Copyright (c) 2021 Xubai Wang
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KdlError {
    #[error("error requesting api")]
    NetworkError { 
        #[from]
        source: reqwest::Error
    },
    #[error("error deserializing")]
    DeserializeError {
        #[from]
        source: serde_json::Error
    },
    #[error("error serializing")]
    SerializeError {
        #[from]
        source: serde_qs::Error
    },
    #[error("error somehow")]
    CodeError {
        code: i32,
        message: String
    },
    #[error("error building args")]
    BuidlerError,
}
