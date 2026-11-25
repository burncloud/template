//! 最小库，用来给共享检查一个可编译、可测试的 crate。

/// 计算两个 `u64` 的和。
///
/// 和超出 `u64` 范围时返回 [`None`]。
///
/// # Examples
///
/// ```
/// use burncloud_template::checked_add;
///
/// assert_eq!(checked_add(2, 2), Some(4));
/// assert_eq!(checked_add(u64::MAX, 1), None);
/// ```
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
