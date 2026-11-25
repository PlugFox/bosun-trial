//! A scratch crate for the worktree-mode trial of bosun (#8).

/// Returns `a` unchanged.
pub fn id(a: i64) -> i64 {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_identity() {
        assert_eq!(id(7), 7);
    }
}
