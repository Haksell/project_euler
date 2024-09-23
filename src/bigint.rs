// TODO: test profusely

use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Debug, Default)]
pub struct BigInt {
    repr: Vec<u64>,
}

impl BigInt {
    pub fn zero(&self) -> Self {
        Self { repr: vec![] }
    }

    pub fn is_zero(&self) -> bool {
        self.repr.is_empty()
    }

    pub fn is_even(&self) -> bool {
        match self.repr.first() {
            Some(d) => d & 1 == 0,
            None => true,
        }
    }

    pub fn is_odd(&self) -> bool {
        match self.repr.first() {
            Some(d) => d & 1 == 1,
            None => false,
        }
    }

    pub fn bit_length(&self) -> usize {
        match self.repr.last() {
            Some(d) => (self.repr.len() << 6) - d.leading_zeros() as usize,
            None => 0,
        }
    }

    // TODO: Karatsuba or similar
    pub fn digits(&self) -> Vec<u8> {
        const BASE: &[u16] = &[6, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1]; // (1<<64).rev()
        let mut digits = vec![];

        for mut n in self.repr.iter().rev().copied() {
            let mut new_digits = vec![0; digits.len() + BASE.len()];
            for (i, d) in digits.iter().enumerate() {
                for (j, b) in BASE.iter().enumerate() {
                    new_digits[i + j] += d * b;
                }
            }

            let mut i = 0;
            while n != 0 {
                new_digits[i] += (n % 10) as u16;
                n /= 10;
                i += 1;
            }

            let mut carry = 0;
            for i in 0..new_digits.len() {
                new_digits[i] += carry;
                carry = new_digits[i] / 10;
                new_digits[i] %= 10;
            }

            while new_digits.last() == Some(&0) {
                new_digits.pop();
            }
            digits = new_digits;
        }

        digits.iter().map(|&d| d as u8).collect()
    }

    // fn remove_trailing_zeros(&mut self) {
    //     while self.repr.last() == Some(&0) {
    //         self.repr.pop();
    //     }
    // }
}

impl From<u64> for BigInt {
    fn from(n: u64) -> Self {
        Self {
            repr: if n == 0 { vec![] } else { vec![n] },
        }
    }
}

impl From<&str> for BigInt {
    fn from(s: &str) -> Self {
        if !s.chars().all(|c| c.is_digit(10)) {
            panic!("Invalid character in string");
        }

        const CHUNK_LEN: usize = 19; // largest number of repr that fit in a u64
        const CHUNK_POW: u64 = 10u64.pow(CHUNK_LEN as u32);

        let mut result = BigInt::from(0u64);
        for i in (0..s.len()).step_by(CHUNK_LEN) {
            let chunk_len = (s.len() - i).min(CHUNK_LEN);
            let chunk = s[i..(i + chunk_len)].parse::<u64>().unwrap();
            let chunk_pow = if chunk_len == CHUNK_LEN {
                CHUNK_POW
            } else {
                10u64.pow(chunk_len as u32)
            };
            result *= chunk_pow;
            result += chunk;
        }
        result
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            write!(f, "0")
        } else {
            self.digits()
                .iter()
                .map(|&d| char::from_digit(d as u32, 10).unwrap())
                .rev()
                .collect::<String>()
                .fmt(f)
        }
    }
}

////////////////////
// u64 OPERATIONS //
////////////////////

impl AddAssign<u64> for BigInt {
    fn add_assign(&mut self, rhs: u64) {
        let mut carry = rhs;
        for i in 0..self.repr.len() {
            let (sum, overflow) = self.repr[i].overflowing_add(carry);
            self.repr[i] = sum;
            carry = overflow as u64;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            self.repr.push(carry);
        }
    }
}

impl Add<u64> for BigInt {
    type Output = Self;

    fn add(self, rhs: u64) -> Self {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

// TODO: Karatsuba or similar
impl MulAssign<u64> for BigInt {
    fn mul_assign(&mut self, rhs: u64) {
        let mut carry = 0u128;
        for i in 0..self.repr.len() {
            let product = (self.repr[i] as u128) * (rhs as u128) + carry;
            self.repr[i] = product as u64;
            carry = product >> 64;
        }
        if carry > 0 {
            self.repr.push(carry as u64);
        }
    }
}

impl Mul<u64> for BigInt {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}
