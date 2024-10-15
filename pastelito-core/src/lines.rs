use crate::{rule::HasSpan, ByteSpan};

/// A range in a document containing line number and character offsets.
pub trait LineCharRange {
    fn new(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Self;
}

#[derive(Default)]
struct LineCounter {
    line_num: usize,
    last_span_start: usize,
    start_char_offset_in_line: usize,
}

impl LineCounter {
    fn span_to_range<R: LineCharRange>(&mut self, text: &str, span: ByteSpan) -> R {
        let start = span.start();
        let end = span.end();

        if start < self.last_span_start {
            panic!("span out of order");
        }

        let (start_line_num, start_char_offset_in_line) = if start == self.last_span_start {
            (self.line_num, self.start_char_offset_in_line)
        } else {
            self.line_num += text[self.last_span_start..start]
                .chars()
                .filter(|&c| c == '\n')
                .count();
            self.last_span_start = start;

            self.start_char_offset_in_line = text[..start]
                .chars()
                .rev()
                .take_while(|&c| c != '\n')
                .count();

            (self.line_num, self.start_char_offset_in_line)
        };

        let end_line_num = start_line_num + text[start..end].chars().filter(|&c| c == '\n').count();
        let end_char_offset_in_line = text[..end].chars().rev().take_while(|&c| c != '\n').count();

        R::new(
            start_line_num as u32,
            start_char_offset_in_line as u32,
            end_line_num as u32,
            end_char_offset_in_line as u32,
        )
    }
}

/// Convert a sequence of items with spans to a sequence of items with ranges.
///
/// `items` must be sorted by span, otherwise this function will panic.
pub fn spans_to_ranges<R: LineCharRange, T: HasSpan, U: Iterator<Item = T>>(
    text: &str,
    items: U,
) -> impl Iterator<Item = (R, T)> + use<'_, R, T, U> {
    let mut counter = LineCounter::default();
    items.map(move |item| {
        let range = counter.span_to_range(text, item.span());
        (range, item)
    })
}
