fn main() {
    let input = "Hello world";
    let image =
        jabcode::write_jabcode(input.as_bytes(), &jabcode::WriteOptions::default()).unwrap();

    let output = jabcode::read_jabcode(&image).unwrap();
    let output = String::from_utf8(output).unwrap();
    println!("{}", output);
    assert_eq!(input, output);
}
