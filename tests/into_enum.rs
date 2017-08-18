
#[macro_use]
extern crate procedurals;

struct A {}
struct B {}

#[derive(IntoEnum)]
enum E {
    A(A),
    B(B),
}

#[test]
fn into_enum() {
    let a = A {};
    let _e: E = a.into();
}
