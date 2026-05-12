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

use crate::{
    db::db_schema_structs::AbstractDbRepr, return_values::carpathia_errors::CarpathiaError,
};

pub(crate) trait DatabaseQuerier {
    fn new(db_url: &str, db_name: &str) -> Result<Self, CarpathiaError>
    where
        Self: Sized;
    async fn get_schema(&self) -> Result<AbstractDbRepr, CarpathiaError>;
}
