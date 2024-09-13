use anyhow::{ensure, Result};

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
    ensure!(
        *point >= MIN_POINT,
        "Points cannot be less than {}: {}",
        MIN_POINT,
        *point,
    );
    ensure!(
        *point <= MAX_POINT,
        "Points cannot be greater than {}: {}",
        MAX_POINT,
        *point,
    );
    ensure!(
        *point % UNIT_POINT == 0,
        "Points must be a multiple of {}: {}",
        UNIT_POINT,
        *point,
    );
    Ok(())
}

pub(crate) fn validate_lizhibang_point(point: &Point) -> Result<()> {
    ensure!(
        *point >= MIN_POINT,
        "Riichi Bet Points cannot be less than {}: {}",
        MIN_POINT,
        *point,
    );
    ensure!(
        *point <= MAX_LIZHIBANG_POINT,
        "Riichi Bet Points cannot be greater than {}: {}",
        MAX_LIZHIBANG_POINT,
        *point,
    );
    ensure!(
        *point % UNIT_POINT == 0,
        "Riichi Bet Points must be a multiple of {}: {}",
        UNIT_POINT,
        *point,
    );
    Ok(())
}

pub(crate) fn validate_changbang_point(point: &Point) -> Result<()> {
    ensure!(
        *point >= MIN_POINT,
        "Repeat Counter Points cannot be less than {}: {}",
        MIN_POINT,
        *point,
    );
    ensure!(
        *point <= MAX_CHANGBANG_POINT,
        "Repeat Counter Points cannot be greater than {}: {}",
        MAX_CHANGBANG_POINT,
        *point,
    );
    ensure!(
        *point % UNIT_POINT == 0,
        "Repeat Counter Points must be a multiple of {}: {}",
        UNIT_POINT,
        *point,
    );
    Ok(())
}

pub(crate) fn validate_ma(ma: &Ma) -> Result<()> {
    ensure!(
        *ma >= MIN_MA,
        "Order Bonus cannot be less than {}: {}",
        MIN_POINT,
        *ma,
    );
    ensure!(
        *ma <= MAX_MA,
        "Order Bonus cannot be greater than {}: {}",
        MAX_POINT,
        *ma,
    );
    Ok(())
}

#[cfg(test)]
mod test {
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
