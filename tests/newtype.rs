
#[macro_use]
extern crate procedurals;

struct Base {}

#[derive(NewType)]
struct New(Base);

#[test]
fn newtype() {
    let b = Base {};
    let _n: New = b.into(); // test of From
}
