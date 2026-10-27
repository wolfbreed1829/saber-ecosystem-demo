//! Educational cost allocation only. USD inputs are exact cents; rates stay rational.
//! No receipts, enrollment, coverage, or payment records are created by this model.
pub type Raw = [String; 9];
pub const MAX_CENTS: u128 = 100_000_000_000_000; // $1 trillion per money input.
pub const MAX_COUNT: u128 = 1_000_000_000;

pub fn defaults() -> Raw {
    [
        "150000000",
        "10000000",
        "10000",
        "2000",
        "0",
        "2",
        "0",
        "0",
        "",
    ]
    .map(str::to_string)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    pub num: u128,
    pub den: u128,
}
impl Ratio {
    pub fn new(num: u128, den: u128) -> Self {
        assert!(den > 0);
        let (mut a, mut b) = (num, den);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self {
            num: num / a,
            den: den / a,
        }
    }
    fn times(self, n: u128) -> Option<Self> {
        Some(Self::new(self.num.checked_mul(n)?, self.den))
    }
    fn plus(self, rhs: Self) -> Option<Self> {
        Some(Self::new(
            self.num
                .checked_mul(rhs.den)?
                .checked_add(rhs.num.checked_mul(self.den)?)?,
            self.den.checked_mul(rhs.den)?,
        ))
    }
    pub fn usd(self) -> String {
        // Half-up at the final cent only. Avoid doubling the remainder.
        let cents =
            self.num / self.den + u128::from(self.num % self.den >= self.den / 2 + self.den % 2);
        let digits = (cents / 100).to_string();
        let mut grouped = String::new();
        for (i, c) in digits.chars().enumerate() {
            if i > 0 && (digits.len() - i).is_multiple_of(3) {
                grouped.push(',');
            }
            grouped.push(c);
        }
        format!("${grouped}.{:02}", cents % 100)
    }
}
pub fn money(cents: u128) -> String {
    Ratio::new(cents, 1).usd()
}

pub fn parse_money(raw: &str) -> Result<u128, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("Enter a dollar amount; enter 0 for none.".into());
    }
    let mut parts = raw.split('.');
    let dollars = parts.next().unwrap_or_default();
    let fraction = parts.next();
    if parts.next().is_some()
        || dollars.is_empty()
        || !dollars.bytes().all(|b| b.is_ascii_digit())
        || fraction
            .is_some_and(|f| f.is_empty() || f.len() > 2 || !f.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err("Use a nonnegative dollar amount with up to 2 decimal places, without commas or symbols.".into());
    }
    let cents = dollars
        .parse::<u128>()
        .ok()
        .and_then(|v| v.checked_mul(100))
        .and_then(|v| {
            let f = fraction.unwrap_or("0");
            v.checked_add(f.parse::<u128>().ok()? * if f.len() == 1 { 10 } else { 1 })
        });
    cents
        .filter(|v| *v <= MAX_CENTS)
        .ok_or_else(|| "Use an amount of $1 trillion or less.".into())
}
pub fn parse_count(raw: &str) -> Result<u128, String> {
    let raw = raw.trim();
    if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
        return Err("Enter a whole number, without commas or signs.".into());
    }
    raw.parse::<u128>()
        .ok()
        .filter(|v| *v <= MAX_COUNT)
        .ok_or_else(|| "Use a count of 1 billion or less.".into())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldError {
    pub field: usize,
    pub message: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inputs {
    pub b: u128,
    pub s: u128,
    pub residents: u128,
    pub e: u128,
    pub d: u128,
    pub h: u128,
    pub k: u128,
    pub g: u128,
}
fn parse(raw: &Raw) -> Result<Inputs, Vec<FieldError>> {
    let mut values = [0; 8];
    let mut errors = Vec::new();
    for i in 0..8 {
        let result = if matches!(i, 2 | 3 | 5 | 6) {
            parse_count(&raw[i])
        } else {
            parse_money(&raw[i])
        };
        match result {
            Ok(v) => values[i] = v,
            Err(message) => errors.push(FieldError { field: i, message }),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let [b, s, residents, e, d, h, k, g] = values;
    let mut error = |field, message: &str| {
        errors.push(FieldError {
            field,
            message: message.into(),
        })
    };
    if residents == 0 {
        error(2, "The community must contain at least 1 resident.");
    }
    if s > b {
        error(
            1,
            "The school budget is included in the total and cannot exceed it.",
        );
    }
    if h == 0 {
        error(5, "Your family must contain at least 1 household member.");
    }
    if h > residents {
        error(
            5,
            "Household members cannot exceed the community's resident population.",
        );
    }
    if e > residents {
        error(
            3,
            "Enrolled students cannot exceed the community's resident population.",
        );
    }
    if k > h || k > e {
        error(
            6,
            "Household students cannot exceed household residents or total enrollment.",
        );
    }
    // The household and its enrolled children are subsets of community residents.
    if h <= residents && k <= e && e - k > residents - h {
        error(
            6,
            "Students outside your household cannot exceed residents outside your household. Check household and community enrollment.",
        );
    }
    if errors.is_empty() {
        Ok(Inputs {
            b,
            s,
            residents,
            e,
            d,
            h,
            k,
            g,
        })
    } else {
        Err(errors)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseholdAnnual {
    pub other: Ratio,
    pub school: Ratio,
    pub support: u128,
    pub total: Ratio,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Results {
    pub input: Inputs,
    pub other_annual: u128,
    pub support: u128,
    pub applied: u128,
    pub surplus: u128,
    pub remaining: u128,
    pub average: Ratio,
    pub other_rate: Ratio,
    pub school_rate: Option<Ratio>,
    pub household_other: Ratio,
    pub household_school: Ratio,
    pub household_total: Ratio,
    pub household_per_resident: Ratio,
    pub household_annual: HouseholdAnnual,
}
fn calculate(i: Inputs) -> Option<Results> {
    let other_annual = i.b.checked_sub(i.s)?;
    let support = i.d.checked_add(i.g.checked_mul(12)?)?;
    let applied = i.s.min(support);
    let surplus = support.saturating_sub(i.s);
    let remaining = i.s.saturating_sub(support);
    let average = Ratio::new(i.b, i.residents.checked_mul(12)?);
    let other_rate = Ratio::new(other_annual, i.residents.checked_mul(12)?);
    let school_rate = if i.e == 0 {
        None
    } else {
        Some(Ratio::new(remaining, i.e.checked_mul(12)?))
    };
    // Shared upkeep is allocated per person, including household children once.
    let household_other = other_rate.times(i.h)?;
    let household_school = if i.k == 0 {
        Ratio::new(0, 1)
    } else {
        school_rate?.times(i.k)?
    };
    let household_total = household_other
        .plus(household_school)?
        .plus(Ratio::new(i.g, 1))?;
    let household_per_resident =
        Ratio::new(household_total.num, household_total.den.checked_mul(i.h)?);
    // Annualize exact rates, never the rounded monthly display strings.
    let household_annual = HouseholdAnnual {
        other: household_other.times(12)?,
        school: household_school.times(12)?,
        support: i.g.checked_mul(12)?,
        total: household_total.times(12)?,
    };
    Some(Results {
        input: i,
        other_annual,
        support,
        applied,
        surplus,
        remaining,
        average,
        other_rate,
        school_rate,
        household_other,
        household_school,
        household_total,
        household_per_resident,
        household_annual,
    })
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comparison {
    pub fees: u128,
    pub funding: u128,
    pub gap: u128,
    pub surplus: u128,
    pub household_total: Ratio,
}
fn compare(r: &Results, f: u128) -> Option<Comparison> {
    let fees = r.input.e.checked_mul(12)?.checked_mul(f)?;
    let funding = fees.checked_add(r.support)?;
    let gap = r.input.s.saturating_sub(funding);
    let surplus = funding.saturating_sub(r.input.s);
    let household_total = r
        .household_other
        .plus(Ratio::new(r.input.k.checked_mul(f)?, 1))?
        .plus(Ratio::new(r.input.g, 1))?;
    Some(Comparison {
        fees,
        funding,
        gap,
        surplus,
        household_total,
    })
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluation {
    pub result: Result<Results, Vec<FieldError>>,
    pub comparison: Result<Option<Comparison>, String>,
}
pub fn evaluate(raw: &Raw) -> Evaluation {
    let result = parse(raw).and_then(|i| {
        calculate(i).ok_or_else(|| {
            vec![FieldError {
                field: 0,
                message:
                    "These values exceed the supported arithmetic range. Reduce the assumptions."
                        .into(),
            }]
        })
    });
    let comparison = if raw[8].trim().is_empty() {
        Ok(None)
    } else {
        parse_money(&raw[8]).and_then(|f| match &result {
            Ok(r) => compare(r, f)
                .map(Some)
                .ok_or_else(|| "This comparison exceeds the supported arithmetic range.".into()),
            Err(_) => Ok(None),
        })
    };
    Evaluation { result, comparison }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raw(edits: &[(usize, &str)]) -> Raw {
        let mut a = defaults();
        for (i, v) in edits {
            a[*i] = v.to_string();
        }
        a
    }
    fn result(edits: &[(usize, &str)]) -> Results {
        evaluate(&raw(edits)).result.unwrap()
    }
    #[test]
    fn per_person_budget_and_default_household() {
        let r = result(&[]);
        assert_eq!(r.input.b, 15_000_000_000);
        assert_eq!(r.input.residents, 10_000);
        assert_eq!(r.average.usd(), "$1,250.00");
        assert_eq!(r.other_annual, 14_000_000_000);
        assert_eq!(r.other_rate.usd(), "$1,166.67");
        assert_eq!(r.household_total.usd(), "$2,333.33");
        assert_eq!(r.household_annual.total.usd(), "$28,000.00");
        assert_eq!(r.school_rate.unwrap().usd(), "$416.67");
    }
    #[test]
    fn household_members_each_receive_one_upkeep_share() {
        for (size, monthly, annual) in [
            ("1", "$1,166.67", "$14,000.00"),
            ("2", "$2,333.33", "$28,000.00"),
            ("5", "$5,833.33", "$70,000.00"),
            ("8", "$9,333.33", "$112,000.00"),
        ] {
            let r = result(&[(5, size)]);
            assert_eq!(r.household_other.usd(), monthly);
            assert_eq!(r.household_annual.other.usd(), annual);
            assert_eq!(r.household_per_resident.usd(), "$1,166.67");
        }
    }
    #[test]
    fn enrollment_updates_family_shares_at_full_precision() {
        for (children, monthly, school, annual, average) in [
            ("0", "$5,833.33", "$0.00", "$70,000.00", "$1,166.67"),
            ("1", "$6,250.00", "$5,000.00", "$75,000.00", "$1,250.00"),
            ("2", "$6,666.67", "$10,000.00", "$80,000.00", "$1,333.33"),
        ] {
            let r = result(&[(5, "5"), (6, children)]);
            assert_eq!(r.household_total.usd(), monthly);
            assert_eq!(r.household_annual.school.usd(), school);
            assert_eq!(r.household_annual.total.usd(), annual);
            assert_eq!(r.household_per_resident.usd(), average);
            assert_eq!(r.input.e, 2000);
            assert_eq!(r.input.b, 15_000_000_000);
        }
    }
    #[test]
    fn resident_population_changes_per_person_rates() {
        let r = result(&[(2, "20000")]);
        assert_eq!(r.average.usd(), "$625.00");
        assert_eq!(r.other_rate.usd(), "$583.33");
        assert_eq!(r.household_other.usd(), "$1,166.67");
        assert_eq!(r.household_annual.total.usd(), "$14,000.00");
    }
    #[test]
    fn school_support_and_optional_gifts_are_counted_once() {
        let r = result(&[(5, "4"), (6, "2"), (4, "2000000")]);
        assert_eq!(r.household_total.usd(), "$5,333.33");
        assert_eq!(r.household_annual.total.usd(), "$64,000.00");
        let r = result(&[(5, "4"), (6, "2"), (7, "50")]);
        assert_eq!(r.support, 60_000);
        assert_eq!(r.household_total.usd(), "$5,549.95");
        assert_eq!(r.household_annual.school.usd(), "$9,999.40");
        assert_eq!(r.household_annual.total.usd(), "$66,599.40");
        assert_eq!(result(&[(7, "50")]).household_total.usd(), "$2,383.33");
    }
    #[test]
    fn fewer_students_and_zero_enrollment() {
        let r = result(&[(3, "1000"), (5, "4"), (6, "2")]);
        assert_eq!(r.school_rate.unwrap().usd(), "$833.33");
        assert_eq!(r.household_annual.total.usd(), "$76,000.00");
        let r = result(&[(3, "0")]);
        assert_eq!(r.school_rate, None);
        assert_eq!(r.remaining, 1_000_000_000);
        assert_eq!(r.household_annual.total.usd(), "$28,000.00");
        let r = result(&[(3, "0"), (4, "10000000")]);
        assert_eq!(r.school_rate, None);
        assert_eq!(r.remaining, 0);
    }
    #[test]
    fn surplus_does_not_discount_other_services() {
        let r = result(&[(4, "11000000"), (5, "4"), (6, "2")]);
        assert_eq!(r.surplus, 100_000_000);
        assert_eq!(r.school_rate.unwrap().usd(), "$0.00");
        assert_eq!(r.household_total.usd(), "$4,666.67");
    }
    #[test]
    fn zero_budgets_and_all_school_budget() {
        assert_eq!(result(&[(0, "0"), (1, "0")]).household_total.usd(), "$0.00");
        assert_eq!(result(&[(1, "0")]).other_rate.usd(), "$1,250.00");
        assert_eq!(result(&[(0, "10000000")]).other_rate.usd(), "$0.00");
    }
    #[test]
    fn comparison_price_is_separate_and_reveals_gaps() {
        for (support, gap) in [("0", 280_000_000), ("2000000", 80_000_000)] {
            let c = evaluate(&raw(&[(4, support), (5, "4"), (6, "2"), (8, "300")]))
                .comparison
                .unwrap()
                .unwrap();
            assert_eq!(c.fees, 720_000_000);
            assert_eq!(c.gap, gap);
            assert_eq!(c.household_total.usd(), "$5,266.67");
        }
        assert_eq!(evaluate(&defaults()).comparison, Ok(None));
        assert_eq!(
            evaluate(&raw(&[(8, "0")])).comparison.unwrap().unwrap().gap,
            1_000_000_000
        );
        let ev = evaluate(&raw(&[(8, "bad")]));
        assert!(ev.result.is_ok());
        assert!(ev.comparison.is_err());
    }
    #[test]
    fn strict_parsing_and_half_up_rounding() {
        assert_eq!(parse_money("0.01"), Ok(1));
        assert_eq!(parse_money("50.5"), Ok(5050));
        for v in [
            "", "NaN", "Infinity", "-1", "1e3", "1,000", "$50", "1.001", "1.", ".5", "1.0.0", "+5",
        ] {
            assert!(parse_money(v).is_err(), "{v}");
        }
        for v in ["", "1.0", "-1", "1e3", "1,000"] {
            assert!(parse_count(v).is_err());
        }
        assert_eq!(Ratio::new(1, 2).usd(), "$0.01");
        assert_eq!(Ratio::new(1, 3).usd(), "$0.00");
    }
    #[test]
    fn population_and_household_subsets_are_compatible() {
        for edits in [
            vec![(2, "0")],
            vec![(1, "150000001")],
            vec![(5, "0")],
            vec![(5, "10001")],
            vec![(3, "10001")],
            vec![(5, "10000")],
            vec![(6, "3")],
            vec![(3, "0"), (6, "1")],
            vec![(2, "1")],
            vec![(2, "1000000001")],
            vec![(0, "1000000000000.01")],
        ] {
            assert!(evaluate(&raw(&edits)).result.is_err(), "{edits:?}");
        }
        // Five residents can contain one four-person household with two students.
        assert!(
            evaluate(&raw(&[(2, "5"), (3, "3"), (5, "4"), (6, "2")]))
                .result
                .is_ok()
        );
        assert!(
            evaluate(&raw(&[(2, "4"), (3, "2"), (5, "4"), (6, "2")]))
                .result
                .is_ok()
        );
    }
    #[test]
    fn supported_limits_do_not_overflow() {
        let ev = evaluate(&raw(&[
            (0, "1000000000000"),
            (1, "1000000000000"),
            (2, "1000000000"),
            (3, "1000000000"),
            (4, "1000000000000"),
            (5, "1000000000"),
            (6, "1000000000"),
            (7, "1000000000000"),
            (8, "1000000000000"),
        ]));
        assert!(ev.result.is_ok());
        assert!(ev.comparison.unwrap().is_some());
    }
    #[test]
    fn exact_reconciliation_over_resident_scenarios() {
        for residents in 1..=20 {
            for e in 0..=residents {
                for d in [0, 100, 10_000, 11_000] {
                    let i = Inputs {
                        b: 150_000,
                        s: 10_000,
                        residents,
                        e,
                        d,
                        h: residents,
                        k: e,
                        g: 7,
                    };
                    let r = calculate(i).unwrap();
                    let charged = r
                        .other_rate
                        .times(residents * 12)
                        .unwrap()
                        .plus(Ratio::new(r.applied, 1))
                        .unwrap();
                    let school = match r.school_rate {
                        Some(s) => s.times(e * 12).unwrap(),
                        None => Ratio::new(r.remaining, 1),
                    };
                    assert_eq!(charged.plus(school).unwrap(), Ratio::new(i.b, 1));
                    assert_eq!(r.applied + r.surplus, r.support);
                    assert_eq!(
                        r.household_annual.total,
                        r.household_total.times(12).unwrap()
                    );
                }
            }
        }
    }
}
