use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TwoFloat {
    pub(crate) hi: f64,
    pub(crate) lo: f64,
}

impl From<f64> for TwoFloat {
    fn from(value: f64) -> TwoFloat {
        TwoFloat { hi: value, lo: 0f64 }
    }
}

impl From<TwoFloat> for f64 {
    fn from(value: TwoFloat) -> f64 {
        value.hi
    }
}

impl<'a> From<&'a TwoFloat> for f64 {
    fn from(value: &'a TwoFloat) -> f64 {
        value.hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;

    randomized_test!(copy_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a;
        assert_eq!(a.hi, b.hi);
        assert_eq!(a.lo, b.lo);
    });

    randomized_test!(clone_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = a.clone();
        assert_eq!(a.hi, b.hi);
        assert_eq!(a.lo, b.lo);
    });

    macro_rules! equality_test {
        ($val_test: ident, $ref_test: ident, $create_values: expr, $assertion: tt) => {
            randomized_test!($val_test, |rng: F64Rand| {
                let (a, b) = $create_values(rng);
                $assertion!(a, b);
            });

            randomized_test!($ref_test, |rng: F64Rand| {
                let (a, b) = $create_values(rng);
                $assertion!(&a, &b);
            });
        };
    }

    equality_test!(equality_test, equality_ref_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        (a, a.clone())
    }, assert_eq);

    equality_test!(inequality_test, inequality_ref_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        (a, loop {
            let b = TwoFloat { hi: rng(), lo: rng() };
            if b.hi != a.hi || b.lo != a.lo { break b; };
        })
    }, assert_ne);

    macro_rules! comparison_test {
        ($val_test: ident, $ref_test: ident, $op: tt, $allow_equal: expr) => {
            randomized_test!($val_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, a $op a);
        
                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, a $op b);
        
                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, a $op c);
            });

            randomized_test!($ref_test, |rng: F64Rand| {
                let a = TwoFloat { hi: rng(), lo: rng() };
                assert_eq!($allow_equal, &a $op &a);
        
                let b = TwoFloat { hi: a.hi, lo: rng() };
                assert_eq!(a.lo $op b.lo, &a $op &b);
        
                let c = TwoFloat { hi: rng(), lo: a.lo };
                assert_eq!(a.hi $op c.hi, &a $op &c);
            });
        };
    }

    comparison_test!(less_than_test, less_than_ref_test, <, false);
    comparison_test!(greater_than_test, greater_than_ref_test, >, false);
    comparison_test!(less_equal_test, less_equal_ref_test, <=, true);
    comparison_test!(greater_equal_test, greater_equal_ref_test, >=, true);

    randomized_test!(from_f64_test, |rng: F64Rand| {
        let source = rng();
        let result: TwoFloat = source.into();
        assert_eq!(result.hi, source);
        assert_eq!(result.lo, 0f64);
    });

    randomized_test!(into_f64_test, |rng: F64Rand| {
        let source = TwoFloat { hi: rng(), lo: rng() };
        let result: f64 = source.into();
        assert_eq!(result, source.hi);
    });

    randomized_test!(into_f64_ref_test, |rng: F64Rand| {
        let source = TwoFloat { hi: rng(), lo: rng() };
        let source_ref = &source;
        let result: f64 = source_ref.into();
        assert_eq!(result, source.hi);
    });
}