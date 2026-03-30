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

use crate::DisplayIntoIter;

/// Implement `Display` for `&[T]` if T is `Display`.
///
/// It outputs at most `limit` elements, excluding those from the 5th to the second-to-last one:
/// - `DisplaySlice{ slice: &[1,2,3,4,5,6], ...}` outputs: `"[1,2,3,4,...,6]"`.
pub struct DisplaySlice<'a, T: fmt::Display> {
    inner: DisplayIntoIter<'a, T, std::slice::Iter<'a, T>>,
}

impl<'a, T: fmt::Display> DisplaySlice<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            inner: DisplayIntoIter::new(slice.iter()),
        }
    }

    pub fn at_most(mut self, limit: Option<usize>) -> Self {
        self.inner = self.inner.at_most(limit);
        self
    }

    pub fn sep(mut self, separator: &'a str) -> Self {
        self.inner = self.inner.sep(separator);
        self
    }

    pub fn braces(mut self, left: &'a str, right: &'a str) -> Self {
        self.inner = self.inner.braces(left, right);
        self
    }

    pub fn ellipsis(mut self, s: &'a str) -> Self {
        self.inner = self.inner.ellipsis(s);
        self
    }

    pub fn elem(mut self, prefix: &'a str, suffix: &'a str) -> Self {
        self.inner = self.inner.elem(prefix, suffix);
        self
    }

    pub fn show_count(mut self) -> Self {
        self.inner = self.inner.show_count();
        self
    }

    pub fn limit(&self) -> usize {
        self.inner.limit()
    }
}

impl<T: fmt::Display> fmt::Display for DisplaySlice<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
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
    fn test_display_slice_ellipsis() {
        let a = vec![1, 2, 3, 4, 5, 6];

        // Custom ellipsis "..."
        assert_eq!("[1,2,3,4,...,6]", a.display().ellipsis("...").to_string());

        // Unicode ellipsis
        assert_eq!(
            "[1,2,3,4,\u{2026},6]",
            a.display().ellipsis("\u{2026}").to_string()
        );

        // Empty ellipsis
        assert_eq!("[1,2,3,4,,6]", a.display().ellipsis("").to_string());

        // limit=0 with custom ellipsis
        assert_eq!("[...]", a.display_n(0).ellipsis("...").to_string());

        // Combined with custom separator
        assert_eq!(
            "[1, 2, 3, 4, ..., 6]",
            a.display().ellipsis("...").sep(", ").to_string()
        );
    }

    #[test]
    fn test_display_slice_elem() {
        let a = vec![1, 2, 3];

        // Quotes
        assert_eq!("['1','2','3']", a.display().elem("'", "'").to_string());

        // Quotes with truncation
        let b = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            "['1','2','3','4',..,'6']",
            b.display().elem("'", "'").to_string()
        );

        // Angle brackets
        assert_eq!("[<1>,<2>,<3>]", a.display().elem("<", ">").to_string());

        // Combined with custom separator
        assert_eq!(
            "['1', '2', '3']",
            a.display().elem("'", "'").sep(", ").to_string()
        );

        // Empty prefix/suffix (default behavior)
        assert_eq!("[1,2,3]", a.display().elem("", "").to_string());
    }

    #[test]
    fn test_display_slice_show_count() {
        let a = vec![1, 2, 3, 4, 5, 6, 7];

        // Basic
        assert_eq!(
            "[1,2,3,4,..(7 total),7]",
            a.display().show_count().to_string()
        );

        // limit=0
        assert_eq!("[..(7 total)]", a.display_n(0).show_count().to_string());

        // limit=1
        assert_eq!("[..(7 total),7]", a.display_n(1).show_count().to_string());

        // No truncation (len <= limit): count not shown
        let c = vec![1, 2, 3];
        assert_eq!("[1,2,3]", c.display().show_count().to_string());

        // Combined with custom ellipsis
        assert_eq!(
            "[1,2,3,4,...(7 total),7]",
            a.display().ellipsis("...").show_count().to_string()
        );

        // Combined with all features
        assert_eq!(
            "{'1', '2', '3', '4', ...(7 total), '7'}",
            a.display()
                .ellipsis("...")
                .show_count()
                .elem("'", "'")
                .sep(", ")
                .braces("{", "}")
                .to_string()
        );
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
