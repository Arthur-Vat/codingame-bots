//! Time budgets.
//!
//! The arena can play games with shorter (or longer) time limits than
//! CodinGame's, to test faster. It tells bots the factor in `CG_TIME_SCALE`;
//! bots multiply their own time budget by [`time_scale`]. CodinGame never
//! sets the variable, so there the factor is 1.

/// Environment variable carrying the time factor.
pub const TIME_SCALE_ENV: &str = "CG_TIME_SCALE";

/// The factor from `CG_TIME_SCALE`, or 1 when it is unset or not a
/// positive number.
pub fn time_scale() -> f64 {
    parse_time_scale(std::env::var(TIME_SCALE_ENV).ok().as_deref())
}

fn parse_time_scale(value: Option<&str>) -> f64 {
    value
        .and_then(|text| text.trim().parse::<f64>().ok())
        .filter(|scale| scale.is_finite() && *scale > 0.0)
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests;
