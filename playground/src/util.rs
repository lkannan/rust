/// # Function: add_five
/// # Description: This function takes a number and adds 5 to it
/// # Parameters:
///    num: u32
/// # Return: u32
/// 
/// # Example:
/// '''
/// let x: u32 = 5;
/// let result = add_five(x);
/// '''
/// 
pub fn add_five(num: u32) -> u32 {
    num + 5
}

pub fn subtract_ten(num: u32) -> u32 {
    num - 10
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_five() {
        let x: u32 = 5;
        let result = add_five(x);
        assert_eq!(
            add_five(x),
            10,
            "We are testing the add_five function with {} and it should return {}",
            x,
            result
        );
        println!(
            "Test add_five excecuted with {} and it should return {}",
            x, result
        )
    }

    #[test]
    fn test_subtract_ten() {
        let x: u32 = 15;
        let result = subtract_ten(x);
        assert_eq!(
            subtract_ten(x),
            5,
            "We are testing the subtract_ten function with {} and it should return {}",
            x,
            result
        );
        println!(
            "Test subtract_ten excecuted with {} and it should return {}",
            x, result
        );
    }
}
