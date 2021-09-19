use crate::__private::Lt;
use crate::project::{InitProj, ProjConstruct, UninitProj};
use crate::{Init, Own, Uninit};

use core::mem;
use core::ptr;

/// Creates a constructor that will initialize an [`Uninit<T>`] by
/// moving the data from the given [`Own<T>`].
pub fn move_from<T>(value: Own<'_, T>) -> impl FnOnce(Uninit<'_, T>) -> Init<'_, T> + '_ {
    move |mut uninit| {
        // Don't drop the data owned by `Own` since it will be moved into `uninit`
        let value = Own::leak(value);
        // SAFETY: `uninit.as_mut_ptr()` returns a valid pointer and `value` is a reference.
        unsafe { uninit.as_mut_ptr().copy_from_nonoverlapping(value, 1) };
        // SAFETY: We just initialized the data pointed by `uninit`.
        unsafe { uninit.assume_init() }
    }
}

/// Creates a constructor that will initialize an [`Uninit<T>`] by
/// calling the provided closure with its projection.
///
/// See also [`ProjConstruct`] and (TODO: mention derive macro)
pub fn proj_fn<T, F>(f: F) -> impl FnOnce(Uninit<'_, T>) -> Init<'_, T>
where
    T: ProjConstruct + ?Sized,
    F: for<'a> FnOnce(Lt<'a>, UninitProj<'a, T>) -> InitProj<'a, T>,
{
    move |uninit| T::proj_construct(uninit, f)
}

/// Creates a constructor that will initialize an `Uninit<[T; N]>` by calling
/// the provided closure for each element, passing its index and the corresponding
/// [`Uninit<T>`].
pub fn array_each<T, F, const N: usize>(
    mut f: F,
) -> impl FnOnce(Uninit<'_, [T; N]>) -> Init<'_, [T; N]>
where
    F: FnMut(usize, Uninit<'_, T>) -> Init<'_, T>,
{
    proj_fn::<[T; N], _>(move |_, proj| {
        let mut idx = 0;
        proj.map(|uninit| {
            let init = f(idx, uninit);
            idx += 1;
            init
        })
    })
    // Alternative `unsafe` way, in case the other one breaks for no reason:
    // move |mut uninit| {
    //     // Can't use `cast` because it requires `U` to be `Sized`.
    //     let ptr = uninit.as_mut_ptr() as *mut [T];
    //     // SAFETY: `ptr` is valid because returned by `uninit.as_mut_ptr()`
    //     // and we never expose the lifetime of the `Uninit`.
    //     let uninit_slice = unsafe { Uninit::from_ptr(ptr) };
    //     let init = uninit_slice.init_with(slice_each(f));
    //     mem::forget(init);
    //     // SAFETY: The existance of `init` ensures the data pointed by `uninit`
    //     // has been initialized.
    //     unsafe { uninit.assume_init() }
    // }
}

/// Creates a constructor that will initialize an `Uninit<[T]>` by calling
/// the provided closure for each element, passing its index and the corresponding
/// [`Uninit<T>`].
pub fn slice_each<T, F>(mut f: F) -> impl FnOnce(Uninit<'_, [T]>) -> Init<'_, [T]>
where
    F: FnMut(usize, Uninit<'_, T>) -> Init<'_, T>,
{
    move |mut uninit| {
        // Invariant: `data` points to a slice of `current` initialized `T`s which
        // `DropOnPanic` logically owns.
        struct DropOnPanic<T> {
            data: *mut T,
            current: usize,
        }

        impl<T> Drop for DropOnPanic<T> {
            fn drop(&mut self) {
                let slice = ptr::slice_from_raw_parts_mut(self.data, self.current);
                // SAFETY: for `DropOnPanic`'s invariant `slice` is a valid slice and the elements
                // are initialized and owned by `self`.
                unsafe { ptr::drop_in_place(slice) };
            }
        }

        let len = uninit.len();
        // SAFETY: `uninit.as_mut_ptr()` is a valid pointer, and currently points to 0
        // initialized elements.
        let mut guard = DropOnPanic {
            data: uninit.as_mut_ptr().cast::<T>(),
            current: 0,
        };
        while guard.current != len {
            // TODO: ZSTs may require `wrapping_add`?

            // SAFETY: guard.current < len, and `guard.data` was created from `self.as_mut_ptr()`
            // which is valid for up to `len` elements, so the resulting `curr_ptr` is in bounds.
            let curr_ptr = unsafe { guard.data.add(guard.current) };
            // SAFETY: We just shown that `curr_ptr` is a valid `ptr`
            // and we never expose the lifetime of the `Uninit`.
            let uninit = unsafe { Uninit::from_ptr(curr_ptr) };
            let init = f(guard.current, uninit);
            mem::forget(init);
            // The existance of `init` guarantees the current element has been initialized.
            guard.current += 1;
        }

        // Now that all the elements are initialized don't drop them.
        mem::forget(guard);

        // SAFETY: The invariant of `DropOnPanic` and `guard.current != len` being false
        // ensures the data pointed by `uninit` has been initialized.
        unsafe { uninit.assume_init() }
    }
}
