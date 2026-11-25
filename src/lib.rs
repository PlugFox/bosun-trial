//! A scratch crate for the worktree-mode trial of bosun (#8).
//!
//! The functions are total: none of them panics.

/// Returns `a` unchanged.
pub fn id(a: i64) -> i64 {
    a
}

/// Returns the sum of `a` and `b`.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Returns the product of `a` and `b`.
pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

/// Returns the negation of `a`, wrapping at `i64::MIN` so it never panics.
pub fn neg(a: i64) -> i64 {
    a.wrapping_neg()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_identity() {
        assert_eq!(id(7), 7);
    }

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn multiplies() {
        assert_eq!(mul(4, 5), 20);
    }

    #[test]
    fn negates() {
        assert_eq!(neg(3), -3);
    }
}
