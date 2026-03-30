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

impl Default for DisplayIteratorOptions<'_> {
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

impl DisplayIteratorOptions<'_> {
    pub(crate) fn limit(&self) -> usize {
        self.limit.unwrap_or(5)
    }
}
