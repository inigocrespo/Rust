#[cfg(test)]
mod tests {
    use super::phrases::phrases::english;
    use super::phrases::phrases::spanish;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn say_hello() {
        assert_eq!(english::say_hello(), "Hello");
    }

    #[test]
    fn say_hola() {
        assert_eq!(spanish::say_hello(), "Hola");
    }
}
