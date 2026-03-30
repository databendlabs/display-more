use std::fmt;

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
    /// The maximum number of elements to display. by default, it is 5.
    limit: Option<usize>,
    /// The separator between elements. by default, it is ",".
    separator: &'a str,
    /// The left brace. by default, it is "[".
    left_brace: &'a str,
    /// The right brace. by default, it is "]".
    right_brace: &'a str,
    /// The ellipsis string. by default, it is "..".
    ellipsis: &'a str,
    /// The prefix for each element. by default, it is "".
    elem_prefix: &'a str,
    /// The suffix for each element. by default, it is "".
    elem_suffix: &'a str,
    /// Whether to show the total count when truncated. by default, it is false.
    show_count: bool,
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
            limit: None,
            separator: ",",
            left_brace: "[",
            right_brace: "]",
            ellipsis: "..",
            elem_prefix: "",
            elem_suffix: "",
            show_count: false,
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

    pub fn ellipsis(mut self, s: &'a str) -> Self {
        self.ellipsis = s;
        self
    }

    pub fn elem(mut self, prefix: &'a str, suffix: &'a str) -> Self {
        self.elem_prefix = prefix;
        self.elem_suffix = suffix;
        self
    }

    pub fn show_count(mut self) -> Self {
        self.show_count = true;
        self
    }

    pub fn limit(&self) -> usize {
        self.limit.unwrap_or(5)
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

        let ell;
        let ellipsis = if self.show_count && truncated {
            ell = format!("{}({len} total)", self.ellipsis);
            &ell
        } else {
            self.ellipsis
        };

        if limit == 0 {
            return write!(f, "{}{ellipsis}{}", self.left_brace, self.right_brace);
        }

        write!(f, "{}", self.left_brace)?;

        let (pre, suf, sep) = (self.elem_prefix, self.elem_suffix, self.separator);

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

        write!(f, "{}", self.right_brace)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::DisplayIntoIter;

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
}
