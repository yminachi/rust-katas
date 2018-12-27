use std::cmp::min;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub struct Rate {
    hourly_rate: u32,
    hour_rate_ends: u32
}

pub fn calculate_total_sitter_payment(start_time: u32, end_time: u32, rates: &[Rate]) -> u32 {
    validate_start_end_times(start_time, end_time);
    let sorted_rates = sort_rates_by_end_time(rates);

    let mut current_hour = hour_including_next_day(start_time);
    let mut current_pay = 0;
    for rate in sorted_rates.iter() {
        let effective_rate_end = min(hour_including_next_day(rate.hour_rate_ends), hour_including_next_day(end_time));

        current_pay += rate.hourly_rate * (effective_rate_end - current_hour);
        current_hour = effective_rate_end
    }

    validate_rate_coverage(current_hour, hour_including_next_day(end_time));

    return current_pay
}

//Part of me is certain there's a better way to sort an array immutably
//But I sure couldn't find it
fn sort_rates_by_end_time(rates: &[Rate]) -> Vec<Rate> {
    let mut rates_copy = vec![Rate { hourly_rate: 0, hour_rate_ends: 0 } ; rates.len()];
    rates_copy.copy_from_slice(rates);

    rates_copy.sort_by( |a, b| {
        hour_including_next_day(a.hour_rate_ends).cmp(&hour_including_next_day(b.hour_rate_ends))
    });

    return rates_copy
}

fn hour_including_next_day(hour: u32) -> u32 {
    return if hour <= 4 { hour + 24 } else { hour }
}

fn validate_rate_coverage(current_hour: u32, effective_end_time: u32) {
    if current_hour < effective_end_time {
        panic!("rates list should include all hours worked")
    }
}

fn validate_start_end_times(start_time: u32, end_time: u32) {
    if (start_time < 17 && start_time > 3) || (end_time > 4 && end_time < 18) {
        panic!("invalid start or end time");
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn given_invalid_start_time_then_panics() {
        calculate_total_sitter_payment(16, 4, &[Rate { hourly_rate: 2, hour_rate_ends: 4 }]);
    }

    #[test]
    #[should_panic]
    fn given_invalid_end_time_then_panics() {
        calculate_total_sitter_payment(17, 5, &[Rate { hourly_rate: 2, hour_rate_ends: 4 }]);
    }

    #[test]
    #[should_panic]
    fn given_start_time_after_end_time_then_panics() {
        calculate_total_sitter_payment(24, 23, &[Rate { hourly_rate: 2, hour_rate_ends: 4 }]);
    }

    #[test]
    fn given_valid_times_and_single_rate_returns_total() {
        let result = calculate_total_sitter_payment(17, 4, &[Rate { hourly_rate: 2, hour_rate_ends: 4 }]);

        assert_eq!(result, 22);
    }

    #[test]
    fn given_am_start_time_and_single_rate_returns_total() {
        let result = calculate_total_sitter_payment(3, 4, &[Rate { hourly_rate: 5, hour_rate_ends: 4 }]);

        assert_eq!(result, 5);
    }

    #[test]
    fn given_pm_end_time_and_single_rate_returns_total() {
        let result = calculate_total_sitter_payment(17, 18, &[Rate { hourly_rate: 9, hour_rate_ends: 4 }]);

        assert_eq!(result, 9);
    }

    #[test]
    fn given_multiple_rates_returns_total() {
        let rates = [
            Rate { hourly_rate: 2, hour_rate_ends: 22 },
            Rate { hourly_rate: 5, hour_rate_ends: 4 }
        ];

        let result = calculate_total_sitter_payment(20, 3, &rates);

        assert_eq!(result, 29);
    }

    #[test]
    fn given_multiple_rates_not_in_chronological_order_returns_total() {
        let rates = [
            Rate { hourly_rate: 5, hour_rate_ends: 4 },
            Rate { hourly_rate: 2, hour_rate_ends: 22 }
        ];

        let result = calculate_total_sitter_payment(20, 3, &rates);

        assert_eq!(result, 29);
    }

    #[test]
    #[should_panic]
    fn given_incomplete_rates_then_panics() {
        let rates = [ Rate { hourly_rate: 5, hour_rate_ends: 1 } ];

        calculate_total_sitter_payment(20, 3, &rates);
    }
}