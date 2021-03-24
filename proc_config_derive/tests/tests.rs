#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-builds.rs");
    t.pass("tests/02-custom-deserializer.rs")
}
