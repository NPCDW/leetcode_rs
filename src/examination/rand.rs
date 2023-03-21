#[cfg(test)]
mod rand_test {
    use rand::Rng;

    #[test]
    fn rand_test() {
        println!("{:?}", rand::thread_rng().gen::<f64>());
    }
}