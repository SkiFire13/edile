use crate::__private::Lt;
use crate::{Init, Uninit};

/// Trait for types which can project an [`Uninit<'a, Self>`] field by field.
///
/// The `T` generic parameter is just used for a hacky workaround for HRTB
/// related issues and should always default to `&'a Self`.
///
/// This trait is not intended to be accessed directly by the user. Use
/// [`UninitProj`] or [`ProjConstruct`].
///
/// [`Uninit<'a, Self>`]: Uninit
#[doc(hidden)]
pub trait WithUninitProj<'a, T = &'a Self> {
    /// The projection itself. This can be used more ergonomically through [`UninitProj`].
    type UninitProj: 'a;
}

/// Trait for types which can project an [`Init<'a, Self>`] field by field.
///
/// The `T` generic parameter is just used for a hacky workaround for HRTB
/// related issues and should always default to `&'a Self`.
///
/// This trait is not intended to be accessed directly by the user. Use
/// [`InitProj`] or [`ProjConstruct`].
///
/// [`Init<'a, Self>`]: Init
#[doc(hidden)]
pub trait WithInitProj<'a, T = &'a Self>: WithUninitProj<'a, T> {
    /// The projection itself. This can be used more ergonomically through [`InitProj`].
    type InitProj: 'a;
}

/// Represents the projection field by field of an [`Uninit<'a, T>`].
///
/// [`Uninit<'a, T>`]: Uninit
pub type UninitProj<'a, T> = <T as WithUninitProj<'a>>::UninitProj;

/// Represents the projection field by field of a [`Init<'a, T>`].
///
/// [`Init<'a, T>`]: Init
pub type InitProj<'a, T> = <T as WithInitProj<'a>>::InitProj;

/// Trait for types that can be initialized field by field.
///
/// [`WithUninitProj`], [`WithInitProj`] and [`ProjConstruct`] are usually implemented
/// in such a way that getting an [`InitProj`] from an [`UninitProj`] that was obtained from
/// an initial [`Uninit`] guarantees that such [`Uninit`] has been initialized and can be
/// [`assume_init`]ed.
///
/// [`assume_init`]: Uninit::assume_init
pub trait ProjConstruct: for<'a> WithInitProj<'a> {
    /// Initializes `uninit` by projecting it field by field and using
    /// `f` to initialize each field.
    fn proj_construct<F>(uninit: Uninit<'_, Self>, f: F) -> Init<'_, Self>
    where
        F: for<'a> FnOnce(Lt<'a>, UninitProj<'a, Self>) -> InitProj<'a, Self>;
}

mod tuples {
    use super::*;
    crate::impl_for_tuples! { ($($ty:ident $idx:tt),+ $(,)?) =>
        impl<'a, $($ty),*> WithUninitProj<'a> for ($($ty,)+) {
            type UninitProj = ($(Uninit<'a, $ty>,)+);
        }

        impl<'a, $($ty),*> WithInitProj<'a> for ($($ty,)+) {
            type InitProj = ($(Init<'a, $ty>,)+);
        }

        impl<$($ty),+> ProjConstruct for ($($ty,)+) {
            fn proj_construct<InitFn>(mut uninit: Uninit<'_, Self>, f: InitFn) -> Init<'_, Self>
            where
                InitFn: for<'a> FnOnce(Lt<'a>, UninitProj<'a, Self>) -> InitProj<'a, Self>
            {
                use core::ptr::addr_of_mut;
                let ptr = uninit.as_mut_ptr();
                // SAFETY: `Uninit`'s invariants ensure that `ptr` is a valid pointer,
                // so it's safe to get a pointer to a field through `addr_of_mut!`.
                // SAFETY: `Uninit`'s invariants ensure that `ptr` is a valid pointer,
                // so the field ones are valid too. Moreover we never expose the lifetime of
                // the `Uninit`s created, so `from_ptr` is safe to call.
                let uninit_proj = unsafe { ($(Uninit::from_ptr(addr_of_mut!((*ptr).$idx)),)+) };
                let init_proj = f(Lt::default(), uninit_proj);
                ::core::mem::forget(init_proj);
                // SAFETY: The existance of `init_proj` ensures each field has been initialized.
                unsafe { uninit.assume_init() }
            }
        }
    }
}

mod arrays {
    use super::*;
    use core::mem;

    impl<'a, T, const N: usize> WithUninitProj<'a> for [T; N] {
        type UninitProj = [Uninit<'a, T>; N];
    }

    impl<'a, T, const N: usize> WithInitProj<'a> for [T; N] {
        type InitProj = [Init<'a, T>; N];
    }

    impl<T, const N: usize> ProjConstruct for [T; N] {
        fn proj_construct<InitFn>(mut uninit: Uninit<'_, Self>, f: InitFn) -> Init<'_, Self>
        where
            InitFn: for<'a> FnOnce(Lt<'a>, UninitProj<'a, Self>) -> InitProj<'a, Self>,
        {
            let mut ptr = uninit.as_mut_ptr().cast::<T>();
            let uninit_proj = [(); N].map(|_| {
                // SAFETY: `ptr` is a valid and aligned pointer since it comes from `uninit.as_mut_ptr()`.
                // The following `ptr.add(1)` will run at most `N-1` times before this, thus yielding
                // all the elements in `uninit`, but not one more.
                let uninit = unsafe { Uninit::from_ptr(ptr) };
                // SAFETY: This will run at most `N` times, so this will be at most one past the end of
                // the original `ptr`, which is sound.
                // TODO: Maybe consider `wrapping_add` for ZST?
                ptr = unsafe { ptr.add(1) };
                uninit
            });
            let init_proj = f(Lt::default(), uninit_proj);
            mem::forget(init_proj);
            // SAFETY: The existance of `init_proj` ensures each element has been initialized.
            unsafe { uninit.assume_init() }
        }
    }
}
