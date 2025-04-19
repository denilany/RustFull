
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla, // 0
    I,     // 1
    V,     // 5
    X,     // 10
    L,     // 50
    C,     // 100
    D,     // 500
    M,     // 1000
}

use crate::RomanDigit::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Cannot convert {} to a single Roman digit", num),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut digits = Vec::new();
        
        // Handle thousands (1000-3999)
        let thousands = num / 1000;
        for _ in 0..thousands {
            digits.push(M);
        }
        num %= 1000;

        // Handle hundreds (100-999)
        let hundreds = num / 100;
        match hundreds {
            9 => {
                digits.push(C);
                digits.push(M);
            }
            4 => {
                digits.push(C);
                digits.push(D);
            }
            5..=8 => {
                digits.push(D);
                for _ in 0..(hundreds - 5) {
                    digits.push(C);
                }
            }
            1..=3 => {
                for _ in 0..hundreds {
                    digits.push(C);
                }
            }
            _ => {}
        }
        num %= 100;

        // Handle tens (10-99)
        let tens = num / 10;
        match tens {
            9 => {
                digits.push(X);
                digits.push(C);
            }
            4 => {
                digits.push(X);
                digits.push(L);
            }
            5..=8 => {
                digits.push(L);
                for _ in 0..(tens - 5) {
                    digits.push(X);
                }
            }
            1..=3 => {
                for _ in 0..tens {
                    digits.push(X);
                }
            }
            _ => {}
        }
        num %= 10;

        // Handle ones (1-9)
        match num {
            9 => {
                digits.push(I);
                digits.push(X);
            }
            4 => {
                digits.push(I);
                digits.push(V);
            }
            5..=8 => {
                digits.push(V);
                for _ in 0..(num - 5) {
                    digits.push(I);
                }
            }
            1..=3 => {
                for _ in 0..num {
                    digits.push(I);
                }
            }
            _ => {}
        }

        RomanNumber(digits)
    }
}
