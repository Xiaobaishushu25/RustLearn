#[derive(Debug, Default)]
struct Student<'a> {
    id: u64,
    name: &'a str,
    sex: bool,
}
#[test]
fn test_default() {
    let student = Student {
        id: 5,
        ..Default::default()
    };
    println!("{:?}", student);
}
