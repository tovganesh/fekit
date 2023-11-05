pub mod molecule;

mod my_math {
    #[allow(dead_code)]
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::my_math::add as add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
