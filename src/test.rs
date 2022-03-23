#[test]
fn uri_test() {
    use super::*;
    println!("{:?}", uri_parse(r#"file:///C:/example/file/"#).unwrap());
    println!("{:?}", uri_parse("base64://abababab").unwrap());
}
