
pub mod io {
    #[warn(dead_code)]
    pub fn output(message: &str) {
        println!("{}", message);
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_output() {
        assert_eq!(3, 3);
    }
}
