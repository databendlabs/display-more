pub(crate) struct DisplayIteratorOptions<'a> {
    /// The maximum number of elements to display. by default, it is 5.
    pub(crate) limit: Option<usize>,
    /// The separator between elements. by default, it is ",".
    pub(crate) separator: &'a str,
    /// The left brace. by default, it is "[".
    pub(crate) left_brace: &'a str,
    /// The right brace. by default, it is "]".
    pub(crate) right_brace: &'a str,
    /// The ellipsis string. by default, it is "..".
    pub(crate) ellipsis: &'a str,
    /// The prefix for each element. by default, it is "".
    pub(crate) elem_prefix: &'a str,
    /// The suffix for each element. by default, it is "".
    pub(crate) elem_suffix: &'a str,
    /// Whether to show the total count when truncated. by default, it is false.
    pub(crate) show_count: bool,
}

impl<'a> Default for DisplayIteratorOptions<'a> {
    fn default() -> Self {
        Self {
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
}

impl<'a> DisplayIteratorOptions<'a> {
    pub(crate) fn limit(&self) -> usize {
        self.limit.unwrap_or(5)
    }
}
