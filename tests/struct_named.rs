use generics::{Generic, Prod, Unit};

trait Accumulate {
    fn acc(self) -> u64;
}

impl Accumulate for u64 {
    fn acc(self) -> u64 {
        self
    }
}

impl Accumulate for Unit {
    fn acc(self) -> u64 {
        0
    }
}

impl<A, B> Accumulate for Prod<A, B>
where
    A: Accumulate,
    B: Accumulate,
{
    fn acc(self) -> u64 {
        let Prod(a, b) = self;
        a.acc() + b.acc()
    }
}

#[derive(Generic)]
struct Foo {
    a: u64,
    b: u64,
}

#[test]
fn struct_named() {
    let foo = Foo { a: 19, b: 23 };

    assert_eq!(foo.into_repr().acc(), 42);
}
