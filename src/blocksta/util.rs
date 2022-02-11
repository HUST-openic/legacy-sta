//TODO: move help funstion under /base to here.
// Math functions.

/// Count the absolute distance between 2 values, always > 0.
#[inline]
fn absolute_error(a: f32, b: f32) -> f32 {
    (a-b).abs()
}

/// Count the relative error between 2 values, always choose the larger one as the base.
#[inline]
fn relative_error(a: f32, b: f32) -> f32 {
    if a == b {
        0.0
    } 
    else if a.abs() > b.abs() {
        ((a - b) / a).abs()
    } 
    else {
        ((a - b) / b).abs()
    }
}

/// Check whether the values are nearly equal by checking absolute error and relative error.
/// Fucntion parameters are pass-by-value.
#[inline]
pub fn nearly_equal(lhs: f32, rhs: f32, abs_err_tol: f32, rel_err_tol: f32) -> bool {
    let abs_err = absolute_error(lhs, rhs);
    let rel_err = relative_error(lhs, rhs);

    (abs_err  <= abs_err_tol) || (rel_err <= rel_err_tol)
}


// GAR: Used but not implemented here.

pub struct OsFormatGuard {
    // Not implemented because.
    // 1. don't know what's this about.
    // 2. not effectively used.
}