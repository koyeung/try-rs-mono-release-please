pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn multiply(left: usize, right: usize) -> usize {
    right * left
}

pub fn div(left: f32, right: f32) -> f32 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
