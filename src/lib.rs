//   Copyright 2019 IPinfo library developers
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

/// Get crate version from cargo at build time.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[macro_use]
mod error;
mod api;
mod ipinfo;

pub use crate::ipinfo::*;
pub use api::*;
pub use error::*;
