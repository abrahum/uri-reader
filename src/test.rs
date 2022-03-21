#[test]
fn uri_test() {
    use super::*;
    let uri = uri_parse(r#"file:///C:/example/file/"#).unwrap();
    println!("{:?}", uri2path(&uri));
    println!("{:?}", uri_parse("base64://abababab").unwrap().authority());
}
