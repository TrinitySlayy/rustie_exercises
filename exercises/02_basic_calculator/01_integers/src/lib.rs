fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the comiler error and make the tests pass.
    let multiplier :u32 = 4;
    a + multiplier * b
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
