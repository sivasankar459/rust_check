pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn sub(a: u32 , b: u32) -> {
	a - b
}

pub fn mul(a: u32 , b: u32) ->{
	a * b
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
