use core::mem::MaybeUninit;

// TODO: Maybe replace this with a `AsStorage` trait and a `Storage` struct
// with its lifetime, then move the `unsafe` from the trait to the function
// that creates the `Storage`?

/// Represents some piece of uninitialized memory.
///
/// This trait is mostly used to work around the fact that `MaybeUninit`
/// can't wrap `!Sized` types, in particular `[T]`. Users should probably
/// not implement this trait.
///
/// # Safety:
/// [`as_mut_ptr`] returns a pointer that will be valid for the lifetime
/// of its `&mut self` parameter.
///
/// [`as_mut_ptr`]: Storage::as_mut_ptr
pub unsafe trait Storage<T: ?Sized> {
    /// Returns a pointer to some memory. The pointer will be valid for
    /// the lifetime of the `&mut self` parameter.
    fn as_mut_ptr(&mut self) -> *mut T;
}

// SAFETY: The pointer returned by `as_mut_ptr` is the same as
// the `&mut self` paremeter, so it will be valid for its lifetime.
unsafe impl<T> Storage<T> for MaybeUninit<T> {
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }
}

// SAFETY: The pointer returned by `as_mut_ptr` is the same as
// the `&mut self` paremeter, so it will be valid for its lifetime.
unsafe impl<T> Storage<[T]> for [MaybeUninit<T>] {
    fn as_mut_ptr(&mut self) -> *mut [T] {
        // TODO: Use `MaybeUninit::slice_as_mut_ptr` when it gets stabilized.

        // Can't use `cast` because it requires `U` to be `Sized`.
        self as *mut [MaybeUninit<T>] as *mut [T]
    }
}
