pub trait ToStringRadix {
    fn to_string_radix<const N: u32>(self) -> String;
}

impl ToStringRadix for f64 {
    fn to_string_radix<const N: u32>(mut self) -> String {
        let mut out = String::new();
        if self < 0.0 {
            out.push('-');
            self = -self;
        }
        let mut whole = self as u32;
        if whole == 0 {
            out.push('0');
        } else {
            while whole > 0 {
                let part = whole % N;
                out.insert(0, char::from_digit(part, 16).unwrap());
                whole /= N;
            }
        }
        let mut fract = self.fract();
        if fract != 0.0 {
            out.push('.');
            while fract > 0.0 {
                fract *= N as f64;
                out.push(char::from_digit(fract.trunc() as u32, 16).unwrap());
                fract = fract.fract();
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use crate::format::ToStringRadix;

    macro_rules! t {
        ($dec: literal, $dest: literal, $radix: literal) => {
            assert_eq!(
                &$dec.to_string_radix::<$radix>(),
                $dest,
                concat!("converting ", $dec, " to base ", $radix)
            );
        };
    }

    #[test]
    fn bin() {
        t!(1.5, "1.1", 2);
        t!(1.25, "1.01", 2);
        t!(1.75, "1.11", 2);
        t!(5.625, "101.101", 2);
    }

    #[test]
    fn hex() {
        t!(1.5, "1.8", 16);
        t!(1.25, "1.4", 16);
        t!(1.75, "1.c", 16);
        t!(5.625, "5.a", 16);
        t!(15.625, "f.a", 16);
        t!(16.625, "10.a", 16);
    }
}
