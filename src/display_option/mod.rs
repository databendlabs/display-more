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

use std::fmt;

/// Wrapper that implements `Display` for `Option<T>` using either `Display` or `Debug` formatting.
///
/// It outputs a literal string `"None"` if it is None. Otherwise it invokes the stored
/// formatting function for T.
pub struct DisplayOption<'a, T> {
    inner: &'a Option<T>,
    fmt_fn: fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
}

impl<T> fmt::Display for DisplayOption<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            None => write!(f, "None"),
            Some(x) => (self.fmt_fn)(x, f),
        }
    }
}

/// Implement `Display` for `Option<T>` if T is `Display`.
///
/// It outputs a literal string `"None"` if it is None. Otherwise it invokes the Display
/// implementation for T.
///
/// # Example
///
/// ```rust
/// use display_more::DisplayOptionExt;
///
/// let option = Some(1);
/// assert_eq!(option.display().to_string(), "1");
/// ```
pub trait DisplayOptionExt<'a, T: fmt::Display> {
    fn display(&'a self) -> DisplayOption<'a, T>;
}

impl<T> DisplayOptionExt<'_, T> for Option<T>
where T: fmt::Display
{
    fn display(&self) -> DisplayOption<T> {
        DisplayOption {
            inner: self,
            fmt_fn: <T as fmt::Display>::fmt,
        }
    }
}

/// Extension trait to format `Option<T>` using `Debug` formatting.
///
/// # Example
///
/// ```rust
/// use display_more::DisplayDebugOptionExt;
///
/// let option = Some("hello");
/// assert_eq!(option.display_debug().to_string(), "\"hello\"");
/// ```
pub trait DisplayDebugOptionExt<'a, T: fmt::Debug> {
    fn display_debug(&'a self) -> DisplayOption<'a, T>;
}

impl<T> DisplayDebugOptionExt<'_, T> for Option<T>
where T: fmt::Debug
{
    fn display_debug(&self) -> DisplayOption<T> {
        DisplayOption {
            inner: self,
            fmt_fn: <T as fmt::Debug>::fmt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_option() {
        let option = Some(1);
        assert_eq!(option.display().to_string(), "1");
    }

    #[test]
    fn test_display_option_none() {
        let option = None::<u64>;
        assert_eq!(option.display().to_string(), "None");
    }

    #[test]
    fn test_debug_option() {
        assert_eq!(Some(1).display_debug().to_string(), "1");
        assert_eq!(None::<u64>.display_debug().to_string(), "None");
        // Debug adds quotes around strings
        assert_eq!(Some("hello").display_debug().to_string(), "\"hello\"");
        // Vec has Debug but not Display
        assert_eq!(Some(vec![1, 2, 3]).display_debug().to_string(), "[1, 2, 3]");
    }
}
