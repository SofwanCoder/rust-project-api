#![feature(trait_alias)]

pub mod configs;
pub mod context;
pub mod database;
pub mod emails;
pub mod helpers;
pub mod rand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
