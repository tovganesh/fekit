pub mod atom;
pub mod bond;
pub mod molecule;
/**
 * Author: V. Ganesh
 * License: MIT
 */
pub mod point;

mod my_math {
    #[allow(dead_code)]
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}

#[cfg(test)]
mod tests {
    use super::my_math::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
