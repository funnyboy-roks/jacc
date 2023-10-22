pub trait ToHex {
    fn to_hex(self) -> String;
}

pub trait ToBin {
    fn to_bin(self) -> String;
}

impl ToHex for f64 {
    fn to_hex(mut self) -> String {
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
                let part = whole & 0xf;
                out.insert(0, char::from_digit(part, 16).unwrap());
                whole >>= 4;
            }
        }
        let mut fract = self.fract();
        if fract != 0.0 {
            out.push('.');
            while fract > 0.0 {
                fract *= 16.0;
                out.push(char::from_digit(fract.trunc() as u32, 16).unwrap());
                fract = fract.fract();
            }
        }
        out
    }
}

impl ToBin for f64 {
    fn to_bin(mut self) -> String {
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
                let part = whole % 2;
                out.push(char::from_digit(part, 2).unwrap());
                whole /= 2;
            }
        }
        let mut fract = self.fract();
        if fract != 0.0 {
            out.push('.');
            while fract > 0.0 {
                fract *= 2.0;
                out.push(char::from_digit(fract.trunc() as u32, 16).unwrap());
                fract = fract.fract();
            }
        }
        out
    }
}
