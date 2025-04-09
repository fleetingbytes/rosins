//! # Rosins
//!
//! Get random integers from random.org.

mod error;

pub use error::Error;
use randomorg::Random;

/// Type alias for results with rosins's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Eager getter
pub fn eager() -> Result<Vec<i32>> {
    let key = "RANDOMORG_DEVKEY_TEST";
    let api_key = std::env::var(key)?;
    let r = Random::new(api_key);
    let response = r.generate_integers(0, 6, 571, true)?;
    Ok(response.result.random.data)
}

/// Lazy getter
pub fn lazy() -> Result<Vec<i32>> {
    let key = "RANDOMORG_DEVKEY_TEST";
    let api_key = std::env::var(key)?;
    let r = Random::new(api_key);
    let integers = r
        .request_integers()
        .min(0)
        .max(6)
        .limit(571)
        .replacement(true)
        .collect::<Vec<i32>>()?;
    Ok(integers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let result = lazy()?;
        println!("{:?}", result);
        Ok(())
    }
}
