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

//! Enhanced display formatting for various Rust types.
//!
//! This crate provides extension traits to add flexible display formatting capabilities
//! for common types including `Option<T>`, `Result<T, E>`, slices, and Unix timestamps.
//!
//! # Examples
//!
//! ## Display Option
//!
//! ```rust
//! use display_more::DisplayOptionExt;
//!
//! let some = Some(42);
//! assert_eq!(some.display().to_string(), "42");
//!
//! let none: Option<i32> = None;
//! assert_eq!(none.display().to_string(), "None");
//! ```
//!
//! ## Display Result
//!
//! ```rust
//! use display_more::DisplayResultExt;
//!
//! let ok = Result::<i32, &str>::Ok(42);
//! assert_eq!(ok.display().to_string(), "Ok(42)");
//!
//! let err = Result::<i32, &str>::Err("error");
//! assert_eq!(err.display().to_string(), "Err(error)");
//! ```
//!
//! ## Display Slice
//!
//! ```rust
//! use display_more::DisplaySliceExt;
//!
//! let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
//! assert_eq!(numbers.display().to_string(), "[1,2,3,4,..,8]");
//! assert_eq!(numbers.display_n(3).to_string(), "[1,2,..,8]");
//! ```
//!
//! ## Display Unix Timestamp
//!
//! ```rust
//! use std::time::Duration;
//! use display_more::DisplayUnixTimeStampExt;
//!
//! let timestamp = Duration::from_millis(1723102819023);
//! assert_eq!(
//!     timestamp.display_unix_timestamp().to_string(),
//!     "2024-08-08T07:40:19.023000Z+0000"
//! );
//! ```

pub mod display_option;
mod display_result;
pub mod display_slice;
pub mod display_unix_epoch;

pub use display_option::DisplayOptionExt;
pub use display_result::DisplayResultExt;
pub use display_slice::DisplaySliceExt;
pub use display_unix_epoch::DisplayUnixTimeStampExt;
