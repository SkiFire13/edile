use crate::Invariant;

use core::fmt;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr;

/// Represents an initialized place. This is mostly used as a token
/// that a given [`Uninit<'a, T>`] has been initialized, however it can also
/// be used as a smart points since it implements [`Deref`] and [`DerefMut`].
///
/// An `Init<T>` logically owns the data it points to so when it is dropped it
/// will drop that too.
pub struct Init<'a, T: ?Sized> {
    data: &'a mut T,
    _invariant_lifetime: Invariant<'a>,
    _phantom_owned: PhantomData<T>,
}

impl<'a, T: ?Sized> Init<'a, T> {
    /// Creates a new `Init<'a, T>` token given a raw pointer.
    ///
    /// # Safety
    /// - `data` must be a valid and aligned pointer.
    /// - the value pointed by `data` must be initialized.
    /// - `data` must logically own the data it points to.
    pub unsafe fn from_raw(data: *mut T) -> Init<'a, T> {
        // SAFETY: The caller promises `data` is a valid pointer and
        // that it points to is initialized.
        let data = unsafe { &mut *data };
        Self {
            data,
            _invariant_lifetime: Invariant::default(),
            _phantom_owned: PhantomData,
        }
    }

    /// Converts an `Init<'a, T>` into a `Pin<Init<'a, T>>`.
    pub fn into_pin(this: Self) -> Pin<Init<'a, T>> {
        // SAFETY: It's not possible to move or replace the insides of a `Pin<Init<'a, T>>`
        // when `T: !Unpin`, moreover code that receives one as a token is not allowed to
        // unpin it, so it's safe to pin it without other requirements.
        unsafe { Pin::new_unchecked(this) }
    }
}

impl<'a, T: ?Sized> Deref for Init<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for Init<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for Init<'a, T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.data) };
    }
}

impl<'a, T: ?Sized> AsRef<T> for Init<'a, T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<'a, T: ?Sized> AsMut<T> for Init<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for Init<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("InitRef").field(&self.data).finish()
    }
}

unsafe impl<'a, T: ?Sized + Send> Send for Init<'a, T> {}
unsafe impl<'a, T: ?Sized + Sync> Sync for Init<'a, T> {}
