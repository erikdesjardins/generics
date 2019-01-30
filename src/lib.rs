//! This library implements datatype generics, à la `GHC.Generics` [\[1\]][1] [\[2\]][2].
//!
//! `Generic` is the core trait upon which this library is built.
//!
//! Using `Generic`, types can be converted to and from a simpler set of types
//! representing their basic structure.
//!
//! Useful operations can be implemented inductively on those simpler types,
//! eliminating the need to write boilerplate for each individual type.
//!
//! [1]: https://wiki.haskell.org/GHC.Generics
//! [2]: http://dreixel.net/research/pdf/gdmh.pdf

#![warn(missing_docs)]

use std::marker::PhantomData;

#[cfg(feature = "generics_derive")]
#[doc(hidden)]
pub use generics_derive::Generic;

/// A bidirectional conversion between a type and its `Repr`.
///
/// This trait should not be implemented by hand; use `#[derive(Generic)]` instead.
///
/// # Examples
///
/// Accumulate the sum of all fields. For simplicity, only supports `u64`.
///
/// ```rust
/// use generics::{Generic, Meta, Prod, Singleton};
///
/// trait Accumulate {
///     fn acc(self) -> u64;
/// }
///
/// impl Accumulate for u64 {
///     fn acc(self) -> u64 {
///         self
///     }
/// }
///
/// impl<A, B> Accumulate for Prod<A, B> where A: Accumulate, B: Accumulate {
///     fn acc(self) -> u64 {
///         let Prod(a, b) = self;
///         a.acc() + b.acc()
///     }
/// }
///
/// impl<I, M> Accumulate for Meta<I, M> where I: Accumulate, M: Singleton {
///     fn acc(self) -> u64 {
///         let Meta(inner, _) = self;
///         inner.acc()
///     }
/// }
///
/// fn accumulate<T>(x: T) -> u64 where T: Generic, T::Repr: Accumulate {
///     Generic::into(x).acc()
/// }
///
/// #[derive(Generic)]
/// struct Foo { a: u64, b: u64 }
///
/// #[derive(Generic)]
/// struct Bar { a: u64, b: u64, c: u64, d: u64 }
///
/// fn main() {
///     let foo = Foo { a: 19, b: 23 };
///     let bar = Bar { a: 1, b: 2, c: 3, d: 4 };
///
///     assert_eq!(accumulate(foo), 42);
///     assert_eq!(accumulate(bar), 10);
/// }
/// ```
pub trait Generic {
    /// This type's generic representation.
    ///
    /// Composed of four main types: `Unit`, `Prod`, `Sum`, `Meta`,
    /// and primitive types which can't be defined in terms of the former.
    type Repr;

    /// Converts `Self` into its generic representation.
    fn into(self: Self) -> Self::Repr;

    /// Constructs `Self` from its generic representation.
    fn from(repr: Self::Repr) -> Self;
}

/// Represents a unit type.
///
/// That is, a constructor with no arguments, e.g. a unit struct or unit enum variant.
///
/// # Examples
///
/// A simplified implementation of `Generic` for a unit struct, ignoring `Meta`.
///
/// This is incomplete and should not be written by hand; use `#[derive(Generic)]` instead.
///
/// ```rust
/// use generics::{Generic, Unit};
///
/// struct Foo;
///
/// impl Generic for Foo {
///     type Repr = Unit;
///     fn into(self) -> Self::Repr {
///         Unit
///     }
///     fn from(repr: Self::Repr) -> Self {
///         let Unit = repr;
///         Foo
///     }
/// }
/// ```
pub struct Unit;

/// Represents a product type.
///
/// That is, a constructor with two arguments, e.g. a struct with at least two fields.
/// Structs with more than two fields are represented as nested `Prod`s.
///
/// # Examples
///
/// A simplified implementation of `Generic` for a struct, ignoring `Meta`.
///
/// This is incomplete and should not be written by hand; use `#[derive(Generic)]` instead.
///
/// ```rust
/// use generics::{Generic, Prod};
///
/// struct Three {
///     one: u8,
///     two: u16,
///     three: u32,
/// };
///
/// impl Generic for Three {
///     type Repr = Prod<u8, Prod<u16, u32>>;
///     fn into(self) -> Self::Repr {
///         Prod(self.one, Prod(self.two, self.three))
///     }
///     fn from(repr: Self::Repr) -> Self {
///         let Prod(one, Prod(two, three)) = repr;
///         Three { one, two, three }
///     }
/// }
/// ```
pub struct Prod<A, B>(pub A, pub B);

/// Represents a sum type.
///
/// That is, a constructor taking one of two types, e.g. an enum with at least two variants.
/// Enums with more than two variants are represented as nested `Sum`s.
///
/// # Examples
///
/// A simplified implementation of `Generic` for an enum, ignoring `Meta`.
///
/// This is incomplete and should not be written by hand; use `#[derive(Generic)]` instead.
///
/// ```rust
/// use generics::{Generic, Sum};
///
/// enum Three {
///     One(u8),
///     Two(u16),
///     Three(u32),
/// }
///
/// impl Generic for Three {
///     type Repr = Sum<u8, Sum<u16, u32>>;
///     fn into(self) -> Self::Repr {
///         match self {
///             Three::One(one) => Sum::Left(one),
///             Three::Two(two) => Sum::Right(Sum::Left(two)),
///             Three::Three(three) => Sum::Right(Sum::Right(three)),
///         }
///     }
///     fn from(repr: Self::Repr) -> Self {
///         match repr {
///             Sum::Left(one) => Three::One(one),
///             Sum::Right(Sum::Left(two)) => Three::Two(two),
///             Sum::Right(Sum::Right(three)) => Three::Three(three),
///         }
///     }
/// }
/// ```
pub enum Sum<L, R> {
    #[allow(missing_docs)]
    Left(L),
    #[allow(missing_docs)]
    Right(R),
}

/// Additional metadata related to a type.
///
/// That is, constructor names, field names, etc.
///
/// Metadata added by the `Generic` custom derive is represented as a zero-sized `Singleton`,
/// so it adds no runtime overhead.
///
/// # Examples
///
/// A theoretical implementation of `Generic` for a unit struct, including metadata.
///
/// This should not be written by hand; use `#[derive(Generic)]` instead.
///
/// ```rust
/// # use std::marker::PhantomData;
/// use generics::{Generic, Meta, Singleton, Unit};
///
/// struct Foo;
///
/// impl Generic for Foo {
///     type Repr = Meta<Unit, Foo_Name>;
///     fn into(self) -> Self::Repr {
///         Meta(Unit, PhantomData)
///     }
///     fn from(repr: Self::Repr) -> Self {
///         let Meta(Unit, _) = repr;
///         Foo
///     }
/// }
///
/// struct Foo_Name;
///
/// impl Singleton for Foo_Name {
///     type T = &'static str;
///     fn get() -> Self::T {
///         "Foo"
///     }
/// }
/// ```
///
/// Sometimes, a generic operation does not care about names and other metadata.
/// In that case, you can simply ignore `Meta` nodes and forward to the inner type.
///
/// ```rust
/// use generics::{Generic, Meta, Singleton};
///
/// trait MyTrait {
///     fn do_something(&self);
/// }
///
/// impl<I, M> MyTrait for Meta<I, M> where I: MyTrait, M: Singleton {
///     fn do_something(&self) {
///         let Meta(inner, _metadata) = self;
///         MyTrait::do_something(inner);
///     }
/// }
/// ```
pub struct Meta<I, M>(pub I, pub PhantomData<M>)
where
    M: Singleton;

/// A zero-sized singleton associated with some data.
///
/// Allows `Meta` to hold additional information about a type without carrying it around at runtime.
pub trait Singleton {
    /// Type of the associated data.
    type T;
    /// Get the associated data.
    fn get() -> Self::T;
}
