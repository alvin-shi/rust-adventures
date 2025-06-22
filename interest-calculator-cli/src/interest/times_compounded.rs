use super::frequency::CompoundingFrequency;

pub fn times_compounded(investment_term: u32, frequency: CompoundingFrequency) -> u32 {
    match frequency {
        CompoundingFrequency::AtMaturity => 1,
        CompoundingFrequency::Yearly => investment_term,
        CompoundingFrequency::Quarterly => investment_term * 4,
        CompoundingFrequency::Monthly => investment_term * 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_maturity_compounded_once() {
        let term = 12;

        let result = times_compounded(term, CompoundingFrequency::AtMaturity);

        assert_eq!(result, 1);
    }

    #[test]
    fn yearly_compounded_term_length() {
        let term = 3;

        let result = times_compounded(term, CompoundingFrequency::Yearly);

        assert_eq!(result, term);
    }

    #[test]
    fn quarterly_compounded_four_times_term_length() {
        let term = 1;

        let result = times_compounded(term, CompoundingFrequency::Quarterly);

        assert_eq!(result, term * 4);
    }

    #[test]
    fn monthly_compounded_twelve_times_term_length() {
        let term = 1;

        let result = times_compounded(term, CompoundingFrequency::Monthly);

        assert_eq!(result, term * 12);
    }
}
