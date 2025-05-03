use anyhow::{bail, ensure, Result};

pub(crate) type Point = i32;

pub(crate) const UNIT_POINT: Point = 100;
pub(crate) const MAX_POINT: Point = 1_000_000;
pub(crate) const MIN_POINT: Point = 0;
pub(crate) const MAX_LIZHIBANG_POINT: Point = 10_000;
pub(crate) const MAX_CHANGBANG_POINT: Point = 10_000;

pub(crate) type Ma = i32;

pub(crate) const MAX_MA: Ma = 10_000;
pub(crate) const MIN_MA: Ma = -10_000;

pub(crate) fn validate_point(point: &Point) -> Result<()> {
    match *point {
        p if p < MIN_POINT => bail!("Points cannot be less than {}: {}", MIN_POINT, p),
        p if p > MAX_POINT => bail!("Points cannot be greater than {}: {}", MAX_POINT, p),
        p if p % UNIT_POINT != 0 => bail!("Points must be a multiple of {}: {}", UNIT_POINT, p),
        _ => Ok(()),
    }
}

pub(crate) fn validate_lizhibang_point(point: &Point) -> Result<()> {
    match *point {
        p if p < MIN_POINT => bail!("Riichi Bet Points cannot be less than {}: {}", MIN_POINT, p),
        p if p > MAX_LIZHIBANG_POINT => bail!(
            "Riichi Bet Points cannot be greater than {}: {}",
            MAX_LIZHIBANG_POINT,
            p
        ),
        p if p % UNIT_POINT != 0 => bail!(
            "Riichi Bet Points must be a multiple of {}: {}",
            UNIT_POINT,
            p
        ),
        _ => Ok(()),
    }
}

pub(crate) fn validate_changbang_point(point: &Point) -> Result<()> {
    match *point {
        p if p < MIN_POINT => bail!(
            "Repeat Counter Points cannot be less than {}: {}",
            MIN_POINT,
            p
        ),
        p if p > MAX_CHANGBANG_POINT => bail!(
            "Repeat Counter Points cannot be greater than {}: {}",
            MAX_CHANGBANG_POINT,
            p
        ),
        p if p % UNIT_POINT != 0 => bail!(
            "Repeat Counter Points must be a multiple of {}: {}",
            UNIT_POINT,
            p
        ),
        _ => Ok(()),
    }
}

pub(crate) fn validate_ma(ma: &Ma) -> Result<()> {
    match *ma {
        m if m < MIN_MA => bail!("Order Bonus cannot be less than {}: {}", MIN_MA, m),
        m if m > MAX_MA => bail!("Order Bonus cannot be greater than {}: {}", MAX_MA, m),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_point() {
        validate_point(&MAX_POINT).unwrap();
        validate_point(&MIN_POINT).unwrap();
        validate_point(&UNIT_POINT).unwrap();

        validate_point(&(MAX_POINT + 1)).unwrap_err();
        validate_point(&(MIN_POINT - 1)).unwrap_err();
        validate_point(&(UNIT_POINT + 1)).unwrap_err();
        validate_point(&(UNIT_POINT - 1)).unwrap_err();
    }

    #[test]
    fn test_validate_lizhibang_point() {
        validate_lizhibang_point(&MAX_LIZHIBANG_POINT).unwrap();
        validate_lizhibang_point(&MIN_POINT).unwrap();
        validate_lizhibang_point(&UNIT_POINT).unwrap();

        validate_lizhibang_point(&(MAX_LIZHIBANG_POINT + 1)).unwrap_err();
        validate_lizhibang_point(&(MIN_POINT - 1)).unwrap_err();
        validate_lizhibang_point(&(UNIT_POINT + 1)).unwrap_err();
        validate_lizhibang_point(&(UNIT_POINT - 1)).unwrap_err();
    }

    #[test]
    fn test_validate_changbang_point() {
        validate_changbang_point(&MAX_CHANGBANG_POINT).unwrap();
        validate_changbang_point(&MIN_POINT).unwrap();
        validate_changbang_point(&UNIT_POINT).unwrap();

        validate_changbang_point(&(MAX_CHANGBANG_POINT + 1)).unwrap_err();
        validate_changbang_point(&(MIN_POINT - 1)).unwrap_err();
        validate_changbang_point(&(UNIT_POINT + 1)).unwrap_err();
        validate_changbang_point(&(UNIT_POINT - 1)).unwrap_err();
    }

    #[test]
    fn test_validate_ma() {
        validate_ma(&MAX_MA).unwrap();
        validate_ma(&MIN_MA).unwrap();

        validate_ma(&(MAX_MA + 1)).unwrap_err();
        validate_ma(&(MIN_MA - 1)).unwrap_err();
    }
}
