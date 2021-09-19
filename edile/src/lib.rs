#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

// TODO: Pinned initialization
// TODO: Alloc extensions
// TODO: Maybe fallible initialization?
// TODO: Doc examples (after derive)
// TODO: Decide policy between elided lifetimes vs '_ vs for<'a>

// TODO: re-export depending on features + std case
#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

pub mod project;

mod ctors;
mod init;
mod local;
#[macro_use]
mod macros;
mod own;
mod storage;
mod uninit;

pub use ctors::*;
pub use init::*;
pub use local::*;
pub use own::*;
pub use storage::*;
pub use uninit::*;

#[cfg(feature = "derive")]
pub use edile_derive::*;

#[doc(hidden)]
pub mod __private {
    pub use core::ops::FnOnce;
    pub use core::pin::Pin;

    /// Private utility struct needed to workaround a limitation of the
    /// compiler where it can't determine whether some lifetime will be costrained
    /// or not by in some edge cases.
    /// See <https://github.com/rust-lang/rust/issues/86702#issuecomment-907627869>
    #[derive(Default)]
    pub struct Lt<'a>(core::marker::PhantomData<&'a ()>);
}

#[derive(Default)]
struct Invariant<'a>(core::marker::PhantomData<fn(&'a ()) -> &'a ()>);

macro_rules! impl_for_tuples {
    (($($rules:tt)*) => $($body:tt)*) => {
        macro_rules! inner { ($($rules)*) => { $($body)* } }
        inner!(A 0);
        inner!(A 0, B 1);
        inner!(A 0, B 1, C 2);
        inner!(A 0, B 1, C 2, D 3);
        inner!(A 0, B 1, C 2, D 3, E 4);
        inner!(A 0, B 1, C 2, D 3, E 4, F 5);
        inner!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
        inner!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
    };
}
pub(crate) use impl_for_tuples;
