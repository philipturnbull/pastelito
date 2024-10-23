use std::{ops::Range, str::SplitWhitespace};

use crate::Word;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct ByteOffset(usize);

/// A span of the underlying input data, represented as byte offsets.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ByteSpan {
    start: ByteOffset,
    end: ByteOffset,
}

impl ByteSpan {
    #[inline]
    fn check_state(&self) {
        assert!(
            self.start.0 <= self.end.0,
            "{} <= {}",
            self.start.0,
            self.end.0
        );
    }

    /// Create a new span from the given start and end byte offsets.
    ///
    /// This is "unchecked" in the sense that it does not verify that the given
    /// byte offsets are part of the underlying input data.
    pub fn new_unchecked(start: usize, end: usize) -> Self {
        Self::new(ByteOffset(start), ByteOffset(end))
    }

    /// Create a new span from a part of the given parent string.
    ///
    /// `child` *must* be a sub-slice of `parent`.
    pub fn new_in_str(parent: &str, child: &str) -> Self {
        let start = child.as_ptr() as usize - parent.as_ptr() as usize;
        let end = start + child.len();
        Self::new_unchecked(start, end)
    }

    /// Create a new span from the given start and end byte offsets.
    ///
    /// Internal use only.
    fn new(start: ByteOffset, end: ByteOffset) -> Self {
        let ret = ByteSpan { start, end };
        ret.check_state();
        ret
    }

    /// Create a new span from the given range.
    pub fn of_range(range: Range<usize>) -> Self {
        ByteSpan::new(ByteOffset(range.start), ByteOffset(range.end))
    }

    #[inline(always)]
    pub fn as_str<'a>(&self, data: &'a str) -> &'a str {
        &data[self.start.0..self.end.0]
    }

    /// Is this span empty?
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the start byte offset.
    pub fn start(&self) -> usize {
        self.start.0
    }

    /// Get the end byte offset.
    pub fn end(&self) -> usize {
        self.end.0
    }
}

/// A span of the underlying input data, represented as byte offsets.
///
/// This struct contains several helper methods for manipulating spans based on
/// the string data. For example, getting the first/last characters, splitting
/// spans based on the contents, etc. These methods keep track of the byte
/// offsets automatically and should be used instead of standard methods like
/// `String::split_at`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FullByteSpan<'a> {
    data: &'a str,
    span: ByteSpan,
}

impl<'a> FullByteSpan<'a> {
    #[inline]
    fn check_state(&self) {
        self.span.check_state();
        assert!(
            self.span.end.0 <= self.data.len(),
            "{} <= {}",
            self.span.end.0,
            self.data.len()
        );
    }

    /// Create a new span from the given start and end byte offsets.
    ///
    /// Internal use only.
    fn new(data: &'a str, start: ByteOffset, end: ByteOffset) -> Self {
        let ret = FullByteSpan {
            data,
            span: ByteSpan::new(start, end),
        };
        ret.check_state();
        ret
    }

    /// Create a new span from the entire document.
    pub fn of_document(data: &'a str) -> Self {
        FullByteSpan::of_range(data, 0..data.len())
    }

    /// Convert a `ByteSpan` to a `FullByteSpan`.
    ///
    /// `data` must be the same as the data that `span` was created from.
    pub fn of_span(data: &'a str, span: ByteSpan) -> Self {
        FullByteSpan { data, span }
    }

    /// Convert a `Range` to a `FullByteSpan`.
    ///
    /// `data` must be the same as the data that `range` was created from.
    pub fn of_range(data: &'a str, range: Range<usize>) -> Self {
        FullByteSpan::new(data, ByteOffset(range.start), ByteOffset(range.end))
    }

    /// Convert a `FullByteSpan` to a `ByteSpan`.
    pub fn as_span(&self) -> ByteSpan {
        self.span
    }

    /// Create a new span from the given start and end byte offsets, with the
    /// same underlying data.
    fn adjust(&self, start: ByteOffset, end: ByteOffset) -> Self {
        FullByteSpan::new(self.data, start, end)
    }

    /// Is this span empty?
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Get the string representation of this span.
    pub fn as_str(&self) -> &'a str {
        self.span.as_str(self.data)
    }

    /// Get the first character of this span.
    fn first(&self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            self.as_str().chars().next()
        }
    }

    /// Get the last character of this span.
    pub fn last(&self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            self.as_str().chars().last()
        }
    }

    /// Split this span into two parts, the first character and the suffix.
    pub fn split_first(&self) -> Option<((char, Self), Self)> {
        let c = self.first()?;
        let c_len = c.len_utf8();
        let split_offset = ByteOffset(self.span.start.0 + c_len);

        Some((
            (c, self.adjust(self.span.start, split_offset)),
            self.adjust(split_offset, self.span.end),
        ))
    }

    /// Split this span into two parts, the prefix and the last character.
    pub fn split_last(&self) -> Option<(Self, (char, Self))> {
        let c = self.last()?;
        let c_len = c.len_utf8();
        let split_offset = ByteOffset(self.span.end.0 - c_len);

        Some((
            self.adjust(self.span.start, split_offset),
            (c, self.adjust(split_offset, self.span.end)),
        ))
    }

    /// Split this span into two parts at the first occurrence of the given
    /// `needle`.
    ///
    /// Returns `None` if the `needle` is not found. Returns the span before
    /// `needle`, and the span after `needle`.
    ///
    /// If you need the span of the `needle`, use `FullByteSpan::split_3`
    /// instead.
    pub fn split2(&self, needle: &str) -> Option<(Self, Self)> {
        if let Some(offset) = self.as_str().find(needle) {
            let prefix_offset = ByteOffset(self.span.start.0 + offset);
            let suffix_offset = ByteOffset(self.span.start.0 + offset + needle.len());

            let prefix_span = self.adjust(self.span.start, prefix_offset);
            let suffix_span = self.adjust(suffix_offset, self.span.end);
            Some((prefix_span, suffix_span))
        } else {
            None
        }
    }

    /// Split this span into three parts at the first occurrence of the given
    /// `needle`.
    ///
    /// Returns `None` if the `needle` is not found. Returns the span before
    /// `needle`, the span of `needle`, and the span after `needle`.
    ///
    /// If you don't need the span of the `needle`, use `FullByteSpan::split_2`
    /// instead.
    pub fn split3(&self, needle: &str) -> Option<(Self, Self, Self)> {
        if let Some(offset) = self.as_str().find(needle) {
            let needle_offset = ByteOffset(self.span.start.0 + offset);
            let suffix_offset = ByteOffset(self.span.start.0 + offset + needle.len());

            let prefix_span = self.adjust(self.span.start, needle_offset);
            let needle_span = self.adjust(needle_offset, suffix_offset);
            let suffix_span = self.adjust(suffix_offset, self.span.end);
            Some((prefix_span, needle_span, suffix_span))
        } else {
            None
        }
    }

    /// Get a span which does not include leading characters that satisfy the
    /// given predeicate.
    pub fn skip_while(&self, predicate: impl Fn(char) -> bool) -> Self {
        let offset = self
            .as_str()
            .char_indices()
            .find(|(_, c)| !predicate(*c))
            .map(|(offset, _)| offset);

        match offset {
            Some(offset) => self.adjust(ByteOffset(self.span.start.0 + offset), self.span.end),
            None => self.adjust(self.span.end, self.span.end),
        }
    }

    /// Get a span with leading and trailing whitespace removed.
    pub fn trim(&self) -> Self {
        let mut trimmed = self.skip_while(|c| c.is_whitespace());

        while !trimmed.is_empty() {
            match trimmed.last() {
                Some(c) if c.is_whitespace() => {
                    trimmed.span.end = ByteOffset(trimmed.span.end.0 - c.len_utf8());
                }
                _ => break,
            }
        }

        trimmed
    }

    /// Split this span into whitespace-separated parts.
    pub fn split_whitespace<'b>(&'b self) -> impl Iterator<Item = FullByteSpan<'a>> + use<'a, 'b> {
        SplitWhitespaceIterator {
            data: self.data,
            sw: self.as_str().split_whitespace(),
        }
    }
}

struct SplitWhitespaceIterator<'a, 'b> {
    data: &'a str,
    sw: SplitWhitespace<'b>,
}

impl<'a> Iterator for SplitWhitespaceIterator<'a, '_> {
    type Item = FullByteSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.sw
            .next()
            .map(|s| FullByteSpan::of_span(self.data, ByteSpan::new_in_str(self.data, s)))
    }
}

impl From<FullByteSpan<'_>> for ByteSpan {
    fn from(span: FullByteSpan) -> Self {
        span.span
    }
}

impl<'a> From<FullByteSpan<'a>> for (&'a str, ByteSpan) {
    fn from(span: FullByteSpan<'a>) -> Self {
        (span.data, span.span)
    }
}

impl<'a> From<&[Word<'a>]> for ByteSpan {
    fn from(words: &[Word<'a>]) -> Self {
        let first = words.first().unwrap();
        let last = words.last().unwrap();

        ByteSpan::new(
            ByteOffset(first.as_offset()),
            ByteOffset(last.as_offset() + last.as_str().len()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn of(s: &str) -> FullByteSpan {
        FullByteSpan::of_document(s)
    }

    #[test]
    fn test_multibyte() {
        let span = of("ó");
        assert_eq!(span.as_str(), "ó");
        assert_eq!(span.span.start.0, 0);
        assert_eq!(span.span.end.0, 2);
    }

    #[test]
    fn test_split_first() {
        let span = of("  abc");
        let span = span.skip_while(|c| c.is_whitespace());
        let ((c, prefix), suffix) = span.split_first().unwrap();
        assert_eq!(c, 'a');
        assert_eq!(prefix.as_str(), "a");
        assert_eq!(suffix.as_str(), "bc");
    }

    #[test]
    fn test_split_last() {
        let span = of("  abc");
        let span = span.skip_while(|c| c.is_whitespace());
        let (prefix, (c, suffix)) = span.split_last().unwrap();
        assert_eq!(prefix.as_str(), "ab");
        assert_eq!(c, 'c');
        assert_eq!(suffix.as_str(), "c");
    }

    #[test]
    fn test_split2() {
        let span = of("  key=value=123");
        let span = span.skip_while(|c| c.is_whitespace());
        let (prefix, suffix) = span.split2("=").unwrap();
        assert_eq!(prefix.as_str(), "key");
        assert_eq!(suffix.as_str(), "value=123");
    }

    #[test]
    fn test_split3() {
        let span = of("  key=value=123");
        let span = span.skip_while(|c| c.is_whitespace());
        let (prefix, needle, suffix) = span.split3("=").unwrap();
        assert_eq!(prefix.as_str(), "key");
        assert_eq!(needle.as_str(), "=");
        assert_eq!(suffix.as_str(), "value=123");
    }

    #[test]
    fn test_skip_while() {
        let span = of("\n");
        let span = span.skip_while(|c| c.is_whitespace());
        assert!(span.is_empty(), "span={:?}", span);
    }

    #[test]
    fn test_trim() {
        assert_eq!(of("").trim().as_str(), "");
        assert_eq!(
            of("abc").trim().as_span(),
            ByteSpan::new(ByteOffset(0), ByteOffset(3))
        );
        assert_eq!(
            of(" abc").trim().as_span(),
            ByteSpan::new(ByteOffset(1), ByteOffset(4))
        );
        assert_eq!(
            of("  abc").trim().as_span(),
            ByteSpan::new(ByteOffset(2), ByteOffset(5))
        );
        assert_eq!(
            of("abc ").trim().as_span(),
            ByteSpan::new(ByteOffset(0), ByteOffset(3))
        );
        assert_eq!(
            of("abc  ").trim().as_span(),
            ByteSpan::new(ByteOffset(0), ByteOffset(3))
        );
        assert_eq!(
            of(" abc ").trim().as_span(),
            ByteSpan::new(ByteOffset(1), ByteOffset(4))
        );
        assert_eq!(
            of("  abc  ").trim().as_span(),
            ByteSpan::new(ByteOffset(2), ByteOffset(5))
        );
    }

    #[test]
    fn test_split_whitespace() {
        let data = " Mary   had\ta\u{2009}little  \n\t lamb";
        let span = of(data);
        let tokens = span.split_whitespace().collect::<Vec<_>>();
        let tokens = tokens.iter().map(|span| span.as_str()).collect::<Vec<_>>();
        assert_eq!(tokens, vec!["Mary", "had", "a", "little", "lamb"]);

        let data = " abc";
        let span = of(data);
        let span = span.skip_while(|c| c.is_whitespace());
        let tokens = span.split_whitespace().collect::<Vec<_>>();
        let tokens = tokens.iter().map(|span| span.as_str()).collect::<Vec<_>>();
        assert_eq!(tokens, vec!["abc"]);
    }
}
