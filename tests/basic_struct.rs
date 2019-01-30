use generics::{Generic, Meta, Prod, Singleton, Unit};

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

impl<I, M> Accumulate for Meta<I, M>
where
    I: Accumulate,
    M: Singleton,
{
    fn acc(self) -> u64 {
        let Meta(inner, _) = self;
        inner.acc()
    }
}

fn accumulate<T>(x: T) -> u64
where
    T: Generic,
    T::Repr: Accumulate,
{
    x.into_repr().acc()
}

#[derive(Generic)]
struct Foo {
    a: u64,
    b: u64,
}

#[derive(Generic)]
struct Bar {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

#[test]
fn basic_test() {
    let foo = Foo { a: 19, b: 23 };
    let bar = Bar {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };

    assert_eq!(accumulate(foo), 42);
    assert_eq!(accumulate(bar), 10);
}
