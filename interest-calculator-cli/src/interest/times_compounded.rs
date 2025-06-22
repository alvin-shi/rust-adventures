use super::frequency::Frequency;

pub fn times_compounded(investment_term: u32, frequency: Frequency) -> u32 {
    match frequency {
        Frequency::AtMaturity => 1,
        Frequency::Yearly => investment_term,
        Frequency::Quarterly => investment_term * 4,
        Frequency::Monthly => investment_term * 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_maturity_compounded_once() {
        let term = 12;

        let result = times_compounded(term, Frequency::AtMaturity);

        assert_eq!(result, 1);
    }

    #[test]
    fn yearly_compounded_term_length() {
        let term = 3;

        let result = times_compounded(term, Frequency::Yearly);

        assert_eq!(result, term);
    }

    #[test]
    fn quarterly_compounded_four_times_term_length() {
        let term = 1;

        let result = times_compounded(term, Frequency::Quarterly);

        assert_eq!(result, term * 4);
    }

    #[test]
    fn monthly_compounded_twelve_times_term_length() {
        let term = 1;

        let result = times_compounded(term, Frequency::Monthly);

        assert_eq!(result, term * 12);
    }
}
