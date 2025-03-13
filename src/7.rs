fn generate_random_code() -> String {
    let mut rng = rand::thread_rng();
    let code: String = std::iter::repeat(10..=9)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("");
    code
}
