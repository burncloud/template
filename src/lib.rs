#[must_use]
pub const fn checked_add(left: u64, right: u64) -> Option<u64> {
    left.checked_add(right)
}

#[cfg(test)]
mod tests {
    use super::checked_add;

    #[test]
    fn adds_two_and_two() {
        assert_eq!(checked_add(2, 2), Some(4));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(checked_add(u64::MAX, 1), None);
    }
}
