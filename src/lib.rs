mod levenstein;
pub use levenstein::*;

#[cfg(test)]
mod tests {
    use crate::{recursive, dynamic_wasteful};

    #[test]
    fn recursive_works() {
        let a = "kitten";
        let b = "sitting";
        assert_eq!(recursive(a, b), 3);
        let c = "aita";
        let d = "äiti";
        assert_eq!(recursive(c, d), 2);
    }

    #[test]
    fn dynamic_wasteful_works() {
        let a = "kitten";
        let b = "sitting";
        assert_eq!(dynamic_wasteful(a, b), 3);
        let c = "aita";
        let d = "äiti";
        assert_eq!(dynamic_wasteful(c, d), 2);
    }
}