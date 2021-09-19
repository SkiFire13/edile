use crate::{Init, Storage, Uninit};

use core::borrow::{Borrow, BorrowMut};
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr;

/// Represents a pointer that owns the data it points to,
/// but not the memory itself.
pub struct Own<'storage, T: ?Sized> {
    data: &'storage mut T,
}

impl<'storage, T: ?Sized> Own<'storage, T> {
    /// Creates a new `Own<'a, T>` token given a raw pointer.
    ///
    /// # Safety
    /// - `data` must be a valid and aligned pointer
    /// - the value pointed by `data` must be initialized.
    /// - `data` must logically own the data it points to.
    pub unsafe fn from_raw(data: *mut T) -> Self {
        let data = unsafe { &mut *data };
        Self { data }
    }

    /// Leaks the data pointed by `this`. Note that he returned reference
    /// won't be able to outlive the memory it points to.
    pub fn leak(this: Self) -> &'storage mut T {
        let this = ManuallyDrop::new(this);
        unsafe { ptr::read(&this.data) }
    }

    /// Converts an `Own<'storage, T>` into a `Pin<Own<'storage, T>>`.
    pub fn into_pin(this: Self) -> Pin<Self> {
        // SAFETY: It's not possible to move or replace the insides of a `Pin<Own<T>>`
        // when `T: !Unpin`, so it's safe to pin it without other requirements.
        unsafe { Pin::new_unchecked(this) }
    }

    /// Creates a new `Own<'storage, T>` given an exclusive reference to some memory
    /// represented by `storage` and a constructor `f`.
    ///
    /// See also [`move_from`] and [`proj_fn`] for constructors other than plain closures.
    ///
    /// [`move_from`]: crate::move_from
    /// [`proj_fn`]: crate::proj_fn
    pub fn new_with<S, F>(storage: &'storage mut S, f: F) -> Self
    where
        S: Storage<T>,
        F: FnOnce(Uninit<'_, T>) -> Init<'_, T>,
    {
        let ptr = storage.as_mut_ptr();
        // SAFETY: `Storage`'s invariants ensure `ptr` is valid and aligned.
        // Moreover we never expose the lifetime of the `Uninit`s created,
        // so `from_ptr` is safe to call.
        let uninit = unsafe { Uninit::from_ptr(ptr) };
        let init = f(uninit);
        mem::forget(init);
        // SAFETY: The existance of `init` ensures each field has been initialized.
        unsafe { Self::from_raw(ptr) }
    }
}

impl<'storage, T> Own<'storage, T> {
    /// Consumes `this`, returning the pointed value.
    pub fn into_inner(this: Self) -> T {
        let this = ManuallyDrop::new(this);
        // SAFETY: `Own<T>` logically owns the `T`, and `ManuallyDrop`
        // ensures we won't drop it.
        unsafe { ptr::read(this.data) }
    }
}

impl<'storage, T: ?Sized> Drop for Own<'storage, T> {
    fn drop(&mut self) {
        // SAFETY: `Own<T>` logically owns the `T`
        unsafe { ptr::drop_in_place(self.data) }
    }
}

impl<'storage, T: ?Sized> Deref for Own<'storage, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}
impl<'storage, T: ?Sized> DerefMut for Own<'storage, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

impl<'storage, T: ?Sized> AsRef<T> for Own<'storage, T> {
    fn as_ref(&self) -> &T {
        self
    }
}
impl<'storage, T: ?Sized> AsMut<T> for Own<'storage, T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}
impl<'storage, T: ?Sized> Borrow<T> for Own<'storage, T> {
    fn borrow(&self) -> &T {
        self
    }
}
impl<'storage, T: ?Sized> BorrowMut<T> for Own<'storage, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

// TODO:
// - Box's traits
// - Destructuring projections
// - Partial Moves
// - Slice draining/other?
