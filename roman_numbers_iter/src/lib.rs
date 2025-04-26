use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl RomanNumber {
    pub fn from(num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut num = num;
        let mut digits = Vec::new();
        let symbols = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        for &(value, symbol) in symbols.iter() {
            while num >= value {
                for c in symbol.chars() {
                    match c {
                        'I' => digits.push(RomanDigit::I),
                        'V' => digits.push(RomanDigit::V),
                        'X' => digits.push(RomanDigit::X),
                        'L' => digits.push(RomanDigit::L),
                        'C' => digits.push(RomanDigit::C),
                        'D' => digits.push(RomanDigit::D),
                        'M' => digits.push(RomanDigit::M),
                        _ => (),
                    }
                }
                num -= value;
            }
        }

        RomanNumber(digits)
    }

    pub fn to_int(&self) -> u32 {
        let mut total = 0;
        let mut prev_value = 0;

        for digit in self.0.iter().rev() {
            let value = match digit {
                RomanDigit::Nulla => 0,
                RomanDigit::I => 1,
                RomanDigit::V => 5,
                RomanDigit::X => 10,
                RomanDigit::L => 50,
                RomanDigit::C => 100,
                RomanDigit::D => 500,
                RomanDigit::M => 1000,
            };

            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }

            prev_value = value;
        }

        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.to_int();
        let next_value = current_value + 1;
        *self = RomanNumber::from(next_value);
        Some(self.clone())
    }
}

impl fmt::Display for RomanDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RomanDigit::Nulla => "Nulla",
                RomanDigit::I => "I",
                RomanDigit::V => "V",
                RomanDigit::X => "X",
                RomanDigit::L => "L",
                RomanDigit::C => "C",
                RomanDigit::D => "D",
                RomanDigit::M => "M",
            }
        )
    }
}

impl fmt::Display for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in &self.0 {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}