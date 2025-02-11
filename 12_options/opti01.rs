fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None // Invalid hour
    } else if hour_of_day >= 22 {
        Some(0) // Ice cream is gone
    } else {
        Some(5) // 5 scoops are left
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // Fix the test by comparing with Some(5), as maybe_icecream returns an Option<u16>.
        let icecreams = maybe_icecream(12);

        assert_eq!(icecreams, Some(5)); // Correct: comparing with Some(5), not just 5.
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
