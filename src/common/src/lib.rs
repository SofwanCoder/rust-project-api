#![feature(trait_alias)]

pub mod configs;
pub mod context;
pub mod database;
pub mod drivers;
pub mod error;
pub mod helpers;
pub mod rand;
pub mod utilities;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
