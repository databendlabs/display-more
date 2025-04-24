// Copyright 2021 Datafuse Labs
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

//! Display more types.
//!
//! # Example
//!
//! ```rust
//! use display_more::DisplayOptionExt;
//!
//! let option = Some(1);
//! assert_eq!(option.display().to_string(), "1");
//! ```

pub mod display_option;
mod display_result;
pub mod display_slice;
pub mod display_unix_epoch;

pub use display_option::DisplayOptionExt;
pub use display_result::DisplayResultExt;
pub use display_slice::DisplaySliceExt;
pub use display_unix_epoch::DisplayUnixTimeStampExt;
