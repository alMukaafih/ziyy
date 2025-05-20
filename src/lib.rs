#![allow(clippy::pedantic)]
pub mod error;
#[macro_use]
pub mod scanner;
pub mod stage_1;
pub mod stage_2;
pub mod stage_3;
pub mod stage_4;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
