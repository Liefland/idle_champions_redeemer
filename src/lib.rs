// TODO (if library)
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_test() {
        assert_eq!(super::add(1, 1), 2);
    }
}
