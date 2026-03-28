pub fn clamp(value: f64, [min, max]: [f64; 2]) -> f64 {
    value.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_within_range() {
        assert_eq!(clamp(5.0, [0.0, 10.0]), 5.0);
    }

    #[test]
    fn clamp_below_min() {
        assert_eq!(clamp(-5.0, [0.0, 10.0]), 0.0);
    }

    #[test]
    fn clamp_above_max() {
        assert_eq!(clamp(15.0, [0.0, 10.0]), 10.0);
    }

    #[test]
    fn clamp_at_boundaries() {
        assert_eq!(clamp(0.0, [0.0, 10.0]), 0.0);
        assert_eq!(clamp(10.0, [0.0, 10.0]), 10.0);
    }

    #[test]
    fn clamp_negative_range() {
        assert_eq!(clamp(0.0, [-10.0, -1.0]), -1.0);
        assert_eq!(clamp(-5.0, [-10.0, -1.0]), -5.0);
        assert_eq!(clamp(-15.0, [-10.0, -1.0]), -10.0);
    }
}
