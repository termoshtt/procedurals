
#[macro_use]
extern crate procedurals;

pub struct Base<A> {
    pub a: A,
}

#[derive(NewType)]
pub struct New<A>(Base<A>);

impl<A> Base<A> {
    fn new(a: A) -> Self {
        Self { a }
    }

    fn func(&self) {
        println!("This is B");
    }

    fn func_mut(&mut self) {
        println!("Get mutable ref of B");
    }
}

#[test]
fn newtype() {
    let b: Base<i32> = Base::new(1);
    let mut n: New<i32> = b.into(); // test From
    n.func(); // test Deref
    n.func_mut(); // test DerefMut
    let _b: Base<i32> = n.into();
}
