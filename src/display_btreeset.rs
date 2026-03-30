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

use std::collections::BTreeSet;
use std::fmt;

use crate::DisplayIntoIter;

/// Implement `Display` for `BTreeSet<T>` if `T` is `Display`.
///
/// It outputs at most `limit` elements, excluding those from the 5th to the second-to-last one:
/// - `DisplayBTreeSet{ set: ... }` outputs: `"[1,2,3,4,..,6]"`.
pub struct DisplayBTreeSet<'a, T: fmt::Display> {
    inner: DisplayIntoIter<'a, T, &'a BTreeSet<T>>,
}

impl<'a, T: fmt::Display> DisplayBTreeSet<'a, T> {
    pub fn new(set: &'a BTreeSet<T>) -> Self {
        Self {
            inner: DisplayIntoIter::new(set),
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

impl<T: fmt::Display> fmt::Display for DisplayBTreeSet<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// Implement `Display` for `BTreeSet<T>` if `T` is `Display`.
///
/// It outputs at most `MAX` elements, excluding those from the 5th to the second-to-last one:
/// - `DisplayBTreeSet([1,2,3,4,5,6])` outputs: `"[1,2,3,4,..,6]"`.
///
/// # Example
///
/// ```rust
/// use std::collections::BTreeSet;
///
/// use display_more::DisplayBTreeSetExt;
///
/// let a = (1..=6).collect::<BTreeSet<_>>();
/// assert_eq!(a.display().to_string(), "[1,2,3,4,..,6]");
/// ```
pub trait DisplayBTreeSetExt<'a, T: fmt::Display> {
    fn display(&'a self) -> DisplayBTreeSet<'a, T>;

    /// Display at most `n` elements.
    fn display_n(&'a self, n: usize) -> DisplayBTreeSet<'a, T> {
        self.display().at_most(Some(n))
    }
}

impl<T> DisplayBTreeSetExt<'_, T> for BTreeSet<T>
where T: fmt::Display
{
    fn display(&self) -> DisplayBTreeSet<'_, T> {
        DisplayBTreeSet::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::DisplayBTreeSet;
    use crate::DisplayBTreeSetExt;

    #[test]
    fn test_display_btreeset() {
        let set = (1..=3).collect::<BTreeSet<_>>();
        let display = DisplayBTreeSet::new(&set);

        assert_eq!(display.to_string(), "[1,2,3]");
    }

    #[test]
    fn test_display_empty_set() {
        let set = BTreeSet::<i32>::new();
        let display = DisplayBTreeSet::new(&set);

        assert_eq!(display.to_string(), "[]");
    }

    #[test]
    fn test_display_btreeset_with_1_item() {
        let set = (1..=1).collect::<BTreeSet<_>>();
        let display = DisplayBTreeSet::new(&set);

        assert_eq!(display.to_string(), "[1]");
    }

    #[test]
    fn test_display_btreeset_limit() {
        let set = (1..=7).collect::<BTreeSet<_>>();

        assert_eq!("[1,2,3,4,..,7]", set.display().to_string());
        assert_eq!("[1,..,7]", set.display_n(2).to_string());
        assert_eq!("[..,7]", set.display_n(1).to_string());
        assert_eq!("[..]", set.display_n(0).to_string());
    }

    #[test]
    fn test_display_btreeset_separator_and_braces() {
        let set = (1..=6).collect::<BTreeSet<_>>();

        assert_eq!("[1, 2, 3, 4, .., 6]", set.display().sep(", ").to_string());
        assert_eq!(
            "{1|..|6}",
            set.display_n(2).sep("|").braces("{", "}").to_string()
        );
        assert_eq!(
            "1,2,3",
            (1..=3)
                .collect::<BTreeSet<_>>()
                .display()
                .braces("", "")
                .to_string()
        );
    }

    #[test]
    fn test_display_btreeset_ellipsis_and_elem() {
        let set = (1..=6).collect::<BTreeSet<_>>();

        assert_eq!("[1,2,3,4,...,6]", set.display().ellipsis("...").to_string());
        assert_eq!(
            "['1','2','3','4',..,'6']",
            set.display().elem("'", "'").to_string()
        );
        assert_eq!("[...]", set.display_n(0).ellipsis("...").to_string());
    }

    #[test]
    fn test_display_btreeset_show_count() {
        let set = (1..=7).collect::<BTreeSet<_>>();

        assert_eq!(
            "[1,2,3,4,..(7 total),7]",
            set.display().show_count().to_string()
        );
        assert_eq!("[..(7 total)]", set.display_n(0).show_count().to_string());
        assert_eq!(
            "{'1', '2', '3', '4', ...(7 total), '7'}",
            set.display()
                .ellipsis("...")
                .show_count()
                .elem("'", "'")
                .sep(", ")
                .braces("{", "}")
                .to_string()
        );
    }

    #[test]
    fn test_display_btreeset_limit_getter_and_large_limit() {
        let set = (1..=3).collect::<BTreeSet<_>>();

        let display = DisplayBTreeSet::new(&set);
        assert_eq!(5, display.limit());
        assert_eq!(10, display.at_most(Some(10)).limit());
        assert_eq!("[1,2,3]", set.display_n(10).to_string());
    }

    #[test]
    fn test_display_btreeset_show_count_without_truncation() {
        let set = (1..=3).collect::<BTreeSet<_>>();
        assert_eq!(
            "[1,2,3]",
            DisplayBTreeSet::new(&set).show_count().to_string()
        );
    }
}
