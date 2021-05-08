

#[warn(dead_code)]
pub fn debug(message: &str) {
    println!("{}", message);
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_debug() {
        assert_eq!(3, 3);
    }
}

