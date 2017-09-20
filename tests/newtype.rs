
#[macro_use]
extern crate procedurals;

struct Base {}

impl Base {
    fn func(&self) {
        println!("This is B");
    }
    fn func_mut(&mut self) {
        println!("Get mutable ref of B");
    }
}

#[derive(NewType)]
struct New(Base);

#[test]
fn newtype() {
    let b = Base {};
    let mut n: New = b.into(); // test From
    n.func(); // test Deref
    n.func_mut(); // test DerefMut
}

struct BaseT<A> {
    a: A,
}

#[derive(NewType)]
struct NewT<A>(Base<A>);
