//! Testing docs

/// Doc test
/// # Examples
/// ```
/// use bimber_ecs::add;
/// let result = add(2, 2);
///
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test(){
        assert_eq!(add(2,2), 4);
    }
}
