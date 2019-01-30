use generics::{Generic, Unit};

trait Accumulate {
    fn acc(self) -> u64;
}

impl Accumulate for Unit {
    fn acc(self) -> u64 {
        13
    }
}

#[derive(Generic)]
struct Foo;

#[test]
fn struct_unit() {
    let foo = Foo;

    assert_eq!(foo.into_repr().acc(), 13);
}
