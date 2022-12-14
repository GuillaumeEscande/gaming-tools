

#[warn(dead_code)]
pub fn output(message: String) {
    println!("{}", message);
}

#[warn(dead_code)]
pub fn debug(message: String) {
    eprintln!("{}", message);
}


