
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Instruction {
    pub fn parse<'a>(line: &'a str) -> Option<Instruction> {
        let mut tokens = line.trim().split_whitespace();
        match tokens.next() {
            Some("rect") => Instruction::parse_rect(tokens),
            Some("rotate") => Instruction::parse_rotate(tokens),
            _ => None,
        }
    }

    /// `rect AxB` turns on all of the pixels in a rectangle at the top-left of the screen which is
    /// A wide and B tall.
    fn parse_rect<'a, I>(mut tokens: I) -> Option<Instruction>
        where I: Iterator<Item = &'a str>
    {
        if let Some(maybe_dimensions) = tokens.next() {
            if let Some(x) = maybe_dimensions.find('x') {
                let (width_s, rest) = maybe_dimensions.split_at(x);
                let (_x, height_s) = rest.split_at(1);

                let width_p = width_s.parse::<usize>().ok();
                let height_p = height_s.parse::<usize>().ok();

                match (width_p, height_p) {
                    (Some(width), Some(height)) => {
                        return Some(Instruction::Rect(width, height));
                    }
                    _ => {}
                };
            };
        };
        None
    }

    /// `rotate row y=A by B` shifts all of the pixels in row A (0 is the top row) right by
    /// B pixels. Pixels that would fall off the right end appear at the left end of the row.
    /// `rotate column x=A by B` shifts all of the pixels in column A (0 is the left column) down
    /// by B pixels. Pixels that would fall off the bottom appear at the top of the column.
    fn parse_rotate<'a, I>(mut tokens: I) -> Option<Instruction>
        where I: Iterator<Item = &'a str>
    {
        match tokens.next() {
            Some("row") => {
                if let Some((row, shift)) = Instruction::parse_rot_rest(tokens) {
                    Some(Instruction::RotateRow(row, shift))
                } else {
                    None
                }
            }
            Some("column") => {
                if let Some((col, shift)) = Instruction::parse_rot_rest(tokens) {
                    Some(Instruction::RotateCol(col, shift))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_rot_rest<'a, I>(mut tokens: I) -> Option<(usize, usize)>
        where I: Iterator<Item = &'a str>
    {
        match tokens.next() {
            Some(maybe_which) => {
                let mw_bytes = maybe_which.as_bytes();
                if mw_bytes[..2] == b"x="[..2] || mw_bytes[..2] == b"y="[..2] {
                    let digits = {
                            unsafe { ::std::str::from_utf8_unchecked(&mw_bytes[2..]) }
                        }
                        .parse::<usize>()
                        .ok();
                    if tokens.next() == Some("by") {
                        if let Some(amount_unparsed) = tokens.next() {
                            match (digits, amount_unparsed.parse::<usize>().ok()) {
                                (Some(which), Some(amount)) => return Some((which, amount)),
                                _ => {}
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use instruction::*;
    use super::super::tests::get_example_instructions;

    #[test]
    fn test_parse_instructions() {
        let expected = vec![
            Instruction::Rect(3, 2),
            Instruction::RotateCol(1, 1),
            Instruction::RotateRow(0, 4),
            Instruction::RotateCol(1, 1),
        ];

        for (line, expect) in get_example_instructions().iter().zip(expected) {
            assert!(Instruction::parse(line) == Some(expect));
        }
    }
}
