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

use crate::display_iterator_options::DisplayIteratorOptions;

/// Implement `Display` for cloneable iter sources that yield `&T`.
///
/// It outputs at most `limit` elements, excluding those from the 5th to the second-to-last one.
pub struct DisplayIntoIter<'a, T, S>
where
    T: fmt::Display + 'a,
    S: Clone + IntoIterator<Item = &'a T>,
    S::IntoIter: DoubleEndedIterator + ExactSizeIterator,
{
    items: S,
    options: DisplayIteratorOptions<'a>,
}

impl<'a, T, S> DisplayIntoIter<'a, T, S>
where
    T: fmt::Display + 'a,
    S: Clone + IntoIterator<Item = &'a T>,
    S::IntoIter: DoubleEndedIterator + ExactSizeIterator,
{
    pub fn new(items: S) -> Self {
        Self {
            items,
            options: DisplayIteratorOptions::default(),
        }
    }

    pub fn at_most(mut self, limit: Option<usize>) -> Self {
        self.options.limit = limit;
        self
    }

    pub fn sep(mut self, separator: &'a str) -> Self {
        self.options.separator = separator;
        self
    }

    pub fn braces(mut self, left: &'a str, right: &'a str) -> Self {
        self.options.left_brace = left;
        self.options.right_brace = right;
        self
    }

    pub fn ellipsis(mut self, s: &'a str) -> Self {
        self.options.ellipsis = s;
        self
    }

    pub fn elem(mut self, prefix: &'a str, suffix: &'a str) -> Self {
        self.options.elem_prefix = prefix;
        self.options.elem_suffix = suffix;
        self
    }

    pub fn show_count(mut self) -> Self {
        self.options.show_count = true;
        self
    }

    pub fn limit(&self) -> usize {
        self.options.limit()
    }
}

impl<'a, T, S> fmt::Display for DisplayIntoIter<'a, T, S>
where
    T: fmt::Display + 'a,
    S: Clone + IntoIterator<Item = &'a T>,
    S::IntoIter: DoubleEndedIterator + ExactSizeIterator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limit = self.limit();
        let len = self.items.clone().into_iter().len();
        let truncated = len > limit;
        let options = &self.options;

        let ell;
        let ellipsis = if options.show_count && truncated {
            ell = format!("{}({len} total)", options.ellipsis);
            &ell
        } else {
            options.ellipsis
        };

        if limit == 0 {
            return write!(f, "{}{ellipsis}{}", options.left_brace, options.right_brace);
        }

        write!(f, "{}", options.left_brace)?;

        let (pre, suf, sep) = (options.elem_prefix, options.elem_suffix, options.separator);

        if truncated {
            let mut iter = self.items.clone().into_iter();

            for _ in 0..(limit - 1) {
                let item = iter.next().unwrap();
                write!(f, "{pre}{item}{suf}{sep}")?;
            }

            write!(f, "{ellipsis}{sep}")?;
            write!(
                f,
                "{pre}{}{suf}",
                self.items.clone().into_iter().next_back().unwrap()
            )?;
        } else {
            for (i, item) in self.items.clone().into_iter().enumerate() {
                if i > 0 {
                    write!(f, "{sep}")?;
                }

                write!(f, "{pre}{item}{suf}")?;
            }
        }

        write!(f, "{}", options.right_brace)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::DisplayIntoIter;

    #[derive(Clone)]
    struct Items<'a, T>(&'a [T]);

    impl<'a, T> IntoIterator for Items<'a, T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    #[test]
    fn test_display_into_iter_slice() {
        let values = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            "[1,2,3,4,..,6]",
            DisplayIntoIter::new(values.iter()).to_string()
        );
    }

    #[test]
    fn test_display_into_iter_btreeset() {
        let values = (1..=6).collect::<BTreeSet<_>>();
        assert_eq!("[1,2,3,4,..,6]", DisplayIntoIter::new(&values).to_string());
    }

    #[test]
    fn test_display_into_iter_custom_container() {
        let values = [1, 2, 3, 4, 5, 6];
        assert_eq!(
            "[1,2,..,6]",
            DisplayIntoIter::new(Items(&values))
                .at_most(Some(3))
                .to_string()
        );
    }

    #[test]
    fn test_display_into_iter_limit_edges() {
        let values = [1, 2, 3, 4, 5, 6];

        assert_eq!(
            "[..]",
            DisplayIntoIter::new(values.iter())
                .at_most(Some(0))
                .to_string()
        );
        assert_eq!(
            "[..,6]",
            DisplayIntoIter::new(values.iter())
                .at_most(Some(1))
                .to_string()
        );
    }

    #[test]
    fn test_display_into_iter_combined_formatting() {
        let values = [1, 2, 3, 4, 5, 6, 7];

        assert_eq!(
            "{'1' | '2' | '3' | '4' | ...(7 total) | '7'}",
            DisplayIntoIter::new(values.iter())
                .ellipsis("...")
                .show_count()
                .elem("'", "'")
                .sep(" | ")
                .braces("{", "}")
                .to_string()
        );
    }
}
