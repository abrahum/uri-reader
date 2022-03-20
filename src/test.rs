#[test]
fn uri_test() {
    use super::*;
    println!(
        "{:?}",
        uri_parse("file:///C:/example/file/").unwrap().path()
    );
    println!("{:?}", uri_parse("base64://abababab").unwrap().authority());
}
