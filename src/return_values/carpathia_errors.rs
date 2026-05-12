// Copyright 2026 Stefan Dörig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ErrorNumber {
    CacheFileError,
    CacheFileReadError,
    ConfigFileError,
    DatabaseConnectionError,
    GenerationError,
    Other,
    Success,
}

#[derive(Debug, Clone)]
pub(crate) struct CarpathiaError {
    pub message: String,
    pub error_type: ErrorNumber,
}

impl fmt::Display for CarpathiaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CarpathiaError: {}", self.message)
    }
}

impl From<ErrorNumber> for i32 {
    fn from(error_type: ErrorNumber) -> i32 {
        match error_type {
            ErrorNumber::CacheFileError => 3,
            ErrorNumber::CacheFileReadError => 4,
            ErrorNumber::ConfigFileError => 2,
            ErrorNumber::DatabaseConnectionError => 5,
            ErrorNumber::GenerationError => 1,
            ErrorNumber::Other => 20,
            ErrorNumber::Success => 0,
        }
    }
}

impl Error for CarpathiaError {}
