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

/// Implement `Display` for `Result<T, E>` if T and E are `Display`.
///
/// It outputs a literal string `"Ok(T)"` if it is Ok. Otherwise it invokes the Display
/// implementation for E.
pub struct DisplayResult<'a, T: fmt::Display, E: fmt::Display>(pub &'a Result<T, E>);

impl<T: fmt::Display, E: fmt::Display> fmt::Display for DisplayResult<'_, T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Ok(t) => write!(f, "Ok({})", t),
            Err(e) => write!(f, "Err({})", e),
        }
    }
}

/// Implement `Display` for `Result<T, E>` if T and E are `Display`.
///
/// It outputs a literal string `"Ok(T)"` if it is Ok. Otherwise it invokes the Display
/// implementation for E.
///
/// # Example
///
/// ```rust
/// use display_more::DisplayResultExt;
///
/// let result = Result::<i32, i32>::Ok(1);
/// assert_eq!(result.display().to_string(), "Ok(1)");
///
/// let result = Result::<i32, i32>::Err(2);
/// assert_eq!(result.display().to_string(), "Err(2)");
/// ```
pub trait DisplayResultExt<'a, T: fmt::Display, E: fmt::Display> {
    fn display(&'a self) -> DisplayResult<'a, T, E>;
}

impl<T: fmt::Display, E: fmt::Display> DisplayResultExt<'_, T, E> for Result<T, E> {
    fn display(&self) -> DisplayResult<'_, T, E> {
        DisplayResult(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_result() {
        let result = Result::<i32, i32>::Ok(1);
        assert_eq!(result.display().to_string(), "Ok(1)");

        let result = Result::<i32, i32>::Err(2);
        assert_eq!(result.display().to_string(), "Err(2)");
    }
}
