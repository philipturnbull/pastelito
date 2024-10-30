use crate::{rule::HasSpan, ByteSpan};

/// A range in a document containing line number and character offsets.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineCharRange {
    pub start_line: u32,
    pub start_char_utf16: u32,
    pub end_line: u32,
    pub end_char_utf16: u32,
}

impl LineCharRange {
    pub fn new(start_line: u32, start_char_utf16: u32, end_line: u32, end_char_utf16: u32) -> Self {
        LineCharRange {
            start_line,
            start_char_utf16,
            end_line,
            end_char_utf16,
        }
    }
}

#[derive(Default)]
struct LineCounter {
    line_num: usize,
    last_span_start_byte: usize,
    start_char_offset_in_line: usize,
}

impl LineCounter {
    fn span_to_range(&mut self, input: &str, span: ByteSpan) -> LineCharRange {
        let start_byte = span.start();
        let end = span.end();

        if start_byte < self.last_span_start_byte {
            panic!("span out of order");
        }

        let (start_line_num, start_char_offset_in_line) = if start_byte == self.last_span_start_byte
        {
            (self.line_num, self.start_char_offset_in_line)
        } else {
            self.line_num += input[self.last_span_start_byte..start_byte]
                .chars()
                .filter(|&c| c == '\n')
                .count();
            self.last_span_start_byte = start_byte;

            self.start_char_offset_in_line = input[..start_byte]
                .chars()
                .rev()
                .take_while(|&c| c != '\n')
                .map(|c| c.len_utf16())
                .sum();

            (self.line_num, self.start_char_offset_in_line)
        };

        let end_line_num = start_line_num
            + input[start_byte..end]
                .chars()
                .filter(|&c| c == '\n')
                .count();
        let end_char_offset_in_line: usize = input[..end]
            .chars()
            .rev()
            .take_while(|&c| c != '\n')
            .map(|c| c.len_utf16())
            .sum();

        LineCharRange::new(
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
pub(crate) fn spans_to_ranges<T: HasSpan, U: Iterator<Item = T>>(
    input: &str,
    items: U,
) -> impl Iterator<Item = (LineCharRange, T)> + use<'_, T, U> {
    let mut counter = LineCounter::default();

    items.map(move |item| {
        let range = counter.span_to_range(input, item.span());
        (range, item)
    })
}

#[cfg(test)]
mod tests {
    use pastelito_model::Tag;

    use crate::{
        block::{
            test::{with_testing_block, TestWord},
            ARBITRARY_STR,
        },
        lines::{spans_to_ranges, LineCharRange},
        rule::{Measurement, Results, Warning},
    };

    #[quickcheck]
    fn warning_ranges_are_sorted(results: Results<'static>) -> bool {
        let warnings = results.iter_warnings();
        let warnings: Vec<(LineCharRange, &Warning)> =
            spans_to_ranges(ARBITRARY_STR, warnings).collect();

        warnings.windows(2).all(|pair| pair[0] <= pair[1])
    }

    #[quickcheck]
    fn measurement_ranges_are_sorted(results: Results<'static>) -> bool {
        let measurements = results.iter_measurements();
        let measurements: Vec<(LineCharRange, &Measurement)> =
            spans_to_ranges(ARBITRARY_STR, measurements).collect();

        measurements.windows(2).all(|pair| pair[0] <= pair[1])
    }

    #[test]
    fn test_spans_to_ranges() {
        let words = &[
            TestWord::Word("one", Tag::CardinalNumber),
            TestWord::Space,
            TestWord::Word("two", Tag::CardinalNumber),
            TestWord::Newline,
            TestWord::Word("three", Tag::CardinalNumber),
            TestWord::Space,
            TestWord::Word("four", Tag::CardinalNumber),
            TestWord::Newline,
        ];
        with_testing_block(words, |input, block| {
            let ranges = spans_to_ranges(input, block.iter())
                .map(|(range, _)| range)
                .collect::<Vec<_>>();
            assert_eq!(
                ranges,
                vec![
                    LineCharRange::new(0, 0, 0, 3),
                    LineCharRange::new(0, 4, 0, 7),
                    LineCharRange::new(1, 0, 1, 5),
                    LineCharRange::new(1, 6, 1, 10),
                ]
            );
        });
    }

    #[test]
    fn test_spans_to_ranges_multibyte() {
        let words = &[
            TestWord::Word("🦕", Tag::CardinalNumber),
            TestWord::Space,
            TestWord::Word("two", Tag::CardinalNumber),
            TestWord::Newline,
            TestWord::Word("🦕", Tag::CardinalNumber),
            TestWord::Space,
            TestWord::Word("four", Tag::CardinalNumber),
            TestWord::Newline,
        ];
        with_testing_block(words, |input, block| {
            let ranges = spans_to_ranges(input, block.iter())
                .map(|(range, _)| range)
                .collect::<Vec<_>>();
            assert_eq!(
                ranges,
                vec![
                    LineCharRange::new(0, 0, 0, 2),
                    LineCharRange::new(0, 3, 0, 6),
                    LineCharRange::new(1, 0, 1, 2),
                    LineCharRange::new(1, 3, 1, 7),
                ]
            );
        });
    }
}
