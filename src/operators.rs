pub struct Mul;
pub struct Add;
pub struct Concat;

pub trait Inverse<RHS, LHS, RESULT> {
    fn inverse(&self, result: RESULT, rhs: RHS) -> Option<LHS>;
}

impl Inverse<u64, u64, u64> for Add {
    fn inverse(&self, result: u64, rhs: u64) -> Option<u64> {
        match result.checked_sub(rhs) {
            Some(lhs) => Some(lhs),
            None => None,
        }
    }
}

impl Inverse<u64, u64, u64> for Mul {
    fn inverse(&self, result: u64, rhs: u64) -> Option<u64> {
        match (result / rhs, result % rhs) {
            (lhd, 0) => Some(lhd),
            _ => None,
        }
    }
}

impl Inverse<u64, u64, u64> for Concat {
    fn inverse(&self, result: u64, rhs: u64) -> Option<u64> {
        let rhs_string = rhs.to_string();
        let result_string = result.to_string();
        result_string.ends_with(&rhs_string).then(|| {
            let lhs_str = &result_string[0..result_string.len() - rhs_string.len()];
            let lhs: u64 = lhs_str.parse().unwrap();
            lhs
        })
    }
}
