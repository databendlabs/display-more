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

/// Implement `Display` for `&[T]` if T is `Display`.
///
/// It outputs at most `limit` elements, excluding those from the 5th to the second-to-last one:
/// - `DisplaySlice{ slice: &[1,2,3,4,5,6], ...}` outputs: `"[1,2,3,4,...,6]"`.
pub struct DisplaySlice<'a, T: fmt::Display> {
    slice: &'a [T],
    /// The maximum number of elements to display. by default, it is 5.
    limit: Option<usize>,
    /// The separator between elements. by default, it is ",".
    separator: &'a str,
    /// The left brace. by default, it is "[".
    left_brace: &'a str,
    /// The right brace. by default, it is "]".
    right_brace: &'a str,
}

impl<'a, T: fmt::Display> DisplaySlice<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            slice,
            limit: None,
            separator: ",",
            left_brace: "[",
            right_brace: "]",
        }
    }

    pub fn at_most(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }

    pub fn sep(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }

    pub fn braces(mut self, left: &'a str, right: &'a str) -> Self {
        self.left_brace = left;
        self.right_brace = right;
        self
    }

    pub fn limit(&self) -> usize {
        self.limit.unwrap_or(5)
    }
}

impl<T: fmt::Display> fmt::Display for DisplaySlice<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limit = self.limit();

        if limit == 0 {
            return write!(f, "{}..{}", self.left_brace, self.right_brace);
        }

        let slice = self.slice;
        let len = slice.len();

        write!(f, "{}", self.left_brace)?;

        let sep = self.separator;

        if len > limit {
            for t in slice[..(limit - 1)].iter() {
                write!(f, "{}{}", t, sep)?;
            }

            write!(f, "..{}", sep)?;
            write!(f, "{}", slice.last().unwrap())?;
        } else {
            for (i, t) in slice.iter().enumerate() {
                if i > 0 {
                    write!(f, "{}", sep)?;
                }

                write!(f, "{}", t)?;
            }
        }

        write!(f, "{}", self.right_brace)
    }
}

/// Implement `Display` for `&[T]` if T is `Display`.
///
/// It outputs at most `MAX` elements, excluding those from the 5th to the second-to-last one:
/// - `DisplaySlice(&[1,2,3,4,5,6])` outputs: `"[1,2,3,4,...,6]"`.
///
/// # Example
///
/// ```rust
/// use display_more::DisplaySliceExt;
///
/// let a = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(a.display().to_string(), "[1,2,3,4,..,6]");
/// ```
pub trait DisplaySliceExt<'a, T: fmt::Display> {
    fn display(&'a self) -> DisplaySlice<'a, T>;

    /// Display at most `n` elements.
    fn display_n(&'a self, n: usize) -> DisplaySlice<'a, T> {
        self.display().at_most(Some(n))
    }
}

impl<T> DisplaySliceExt<'_, T> for [T]
where T: fmt::Display
{
    fn display(&self) -> DisplaySlice<T> {
        DisplaySlice::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::DisplaySlice;
    use crate::DisplaySliceExt;

    #[test]
    fn test_display_slice() {
        let a = vec![1, 2, 3, 4];
        assert_eq!("[1,2,3,4]", DisplaySlice::new(&a).to_string());

        let a = vec![1, 2, 3, 4, 5];
        assert_eq!("[1,2,3,4,5]", DisplaySlice::new(&a).to_string());

        let a = vec![1, 2, 3, 4, 5, 6];
        assert_eq!("[1,2,3,4,..,6]", DisplaySlice::new(&a).to_string());

        let a = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!("[1,2,3,4,..,7]", DisplaySlice::new(&a).to_string());

        // with limit

        let a = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(
            "[1,..,7]",
            DisplaySlice::new(&a).at_most(Some(2)).to_string()
        );

        assert_eq!("[1,..,7]", a.display().at_most(Some(2)).to_string());

        assert_eq!("[1,..,7]", a.display_n(2).to_string());

        assert_eq!("[..,7]", a.display_n(1).to_string());

        assert_eq!("[..]", a.display_n(0).to_string());
    }

    #[test]
    fn test_display_slice_separator() {
        let a = vec![1, 2, 3];
        assert_eq!("[1, 2, 3]", a.display().sep(", ").to_string());

        let a = vec![1, 2, 3, 4, 5, 6];
        assert_eq!("[1, 2, 3, 4, .., 6]", a.display().sep(", ").to_string());

        assert_eq!("[1|..|6]", a.display_n(2).sep("|").to_string());

        assert_eq!("[1 2 3 4 .. 6]", a.display().sep(" ").to_string());

        assert_eq!("[1234..6]", a.display().sep("").to_string());
        assert_eq!("[1..6]", a.display_n(2).sep("").to_string());

        // limit=1 with custom separator
        assert_eq!("[.. 6]", a.display_n(1).sep(" ").to_string());

        // limit=0 is unaffected by separator
        assert_eq!("[..]", a.display_n(0).sep(" ").to_string());
    }

    #[test]
    fn test_display_slice_braces() {
        let a = vec![1, 2, 3];

        // Custom braces, no truncation
        assert_eq!("{1,2,3}", a.display().braces("{", "}").to_string());

        // Custom braces with truncation
        let b = vec![1, 2, 3, 4, 5, 6];
        assert_eq!("{1,2,3,4,..,6}", b.display().braces("{", "}").to_string());

        // Custom braces combined with custom separator
        assert_eq!(
            "{1, 2, 3, 4, .., 6}",
            b.display().braces("{", "}").sep(", ").to_string()
        );

        // Custom braces with limit=0
        assert_eq!("{..}", b.display_n(0).braces("{", "}").to_string());

        // Parentheses
        assert_eq!("(1,2,3)", a.display().braces("(", ")").to_string());

        // Angle brackets
        assert_eq!("<1,2,3>", a.display().braces("<", ">").to_string());

        // Empty braces
        assert_eq!("1,2,3", a.display().braces("", "").to_string());
    }
}
