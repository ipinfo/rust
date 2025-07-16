//   Copyright 2019-2024 IPinfo library developers
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

//! IPInfo Utility Functions
use std::time::Duration;

pub const BATCH_MAX_SIZE: u64 = 1000;
pub const BATCH_REQ_TIMEOUT_DEFAULT: Duration = Duration::from_secs(5);

const CACHE_KEY_VERSION: &str = "1";
pub fn cache_key(k: &str) -> String {
    format!("{k}:{CACHE_KEY_VERSION}")
}
