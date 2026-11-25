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
}
