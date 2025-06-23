use super::frequency::CompoundingFrequency;

fn calculate_compound_rate(
    annual_rate: f32,
    term_length: u32,
    frequency: CompoundingFrequency,
) -> f32 {
    match frequency {
        CompoundingFrequency::AtMaturity => annual_rate * term_length as f32,
        CompoundingFrequency::Yearly => annual_rate,
        CompoundingFrequency::Quarterly => annual_rate / 4.0,
        CompoundingFrequency::Monthly => annual_rate / 12.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_maturity_compound_rate_is_rate_times_term_length() {
        let rate: f32 = 5.50;
        let term: u32 = 3;
        let frequency = CompoundingFrequency::AtMaturity;

        let result = calculate_compound_rate(rate, term, frequency);

        assert_eq!(result, 16.5);
    }

    #[test]
    fn yearly_compound_rate_is_annual_rate() {
        let rate: f32 = 5.50;
        let term: u32 = 3;
        let frequency = CompoundingFrequency::Yearly;

        let result = calculate_compound_rate(rate, term, frequency);

        assert_eq!(result, rate);
    }

    #[test]
    fn quarterly_compound_rate_is_a_quarter_of_annual_rate() {
        let rate: f32 = 5.0;
        let term: u32 = 3;
        let frequency = CompoundingFrequency::Quarterly;

        let result = calculate_compound_rate(rate, term, frequency);

        assert_eq!(result, 1.25);
    }

    #[test]
    fn monthly_compound_rate_is_one_tweleth_of_annual_rate() {
        let rate: f32 = 5.0;
        let term: u32 = 3;
        let frequency = CompoundingFrequency::Monthly;

        let result = calculate_compound_rate(rate, term, frequency);

        assert_eq!(result, 5.0 / 12.0);
    }
}
