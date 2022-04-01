pub fn normalize_input(input: String) -> String {
    String::from(
        input
            .strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap(),
    )
}
