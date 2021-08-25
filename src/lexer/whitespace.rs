/// For when you're starting with non-whitespace
#[inline]
pub fn seek(data: &'_ str) -> usize {
    let mut s = 0;

    for c in data.as_bytes() {
        if *c == 0x20 {
            break;
        } else {
            s += 1;
        }
    }

    s
}

/// For when you're starting with whitespace
#[inline]
pub fn skip(data: &'_ str) -> usize {
    let mut s = 0;

    for c in data.as_bytes() {
        if *c == 0x20 {
            s += 1;
        } else {
            break;
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::lexer::whitespace;

    #[test]
    fn skip() {
        let a = "  .";
        let b = " . ";
        let c = ".  ";

        assert_eq!(whitespace::skip(a), 2);
        assert_eq!(whitespace::skip(b), 1);
        assert_eq!(whitespace::skip(c), 0);
    }

    #[test]
    fn seek() {
        assert_eq!(whitespace::seek("  if"), 0);
        assert_eq!(whitespace::seek("if "), 2);
    }
}