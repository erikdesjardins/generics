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
struct Foo<T> {
    a: T,
    b: T,
}

#[derive(Generic)]
struct FooExistingWhereClause<T>
where
    T: std::fmt::Display,
{
    a: T,
    b: T,
}

#[test]
fn struct_generic() {
    let foo = Foo { a: 19u64, b: 23 };
    let foo2 = FooExistingWhereClause { a: 19u64, b: 23 };

    assert_eq!(foo.into_repr().acc(), 42);
    assert_eq!(foo2.into_repr().acc(), 42);
}
