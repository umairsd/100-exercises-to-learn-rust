pub fn hello_world() {
    println!("Hello, packages!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
