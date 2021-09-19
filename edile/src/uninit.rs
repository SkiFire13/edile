use crate::{Init, Invariant};

use core::any::type_name;
use core::fmt;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

/// Represents some unitialized place that needs to be initialized.
pub struct Uninit<'a, T: ?Sized> {
    data: NonNull<T>,
    _invariant_lifetime: Invariant<'a>,
}

impl<'a, T> Uninit<'a, T> {
    /// Returns a reference to the underlying memory.
    pub fn as_maybeuninit(&mut self) -> &mut MaybeUninit<T> {
        // TODO: Use `NonNull::as_uninit_mut` when it gets stabilized.

        let ptr = self.as_mut_ptr().cast::<MaybeUninit<T>>();
        // SAFETY: `self.as_mut_ptr()` returns a valid pointer and `MaybeUninit` doesn't
        // need to be initialized.
        unsafe { &mut *ptr }
    }

    /// Initializes `self` with the given `value`.
    ///
    /// Returns a token that guarantees this place has been initialized.
    pub fn init(mut self, value: T) -> Init<'a, T> {
        // SAFETY: `self.as_mut_ptr()` returns a valid pointer.
        unsafe { self.as_mut_ptr().write(value) };
        // SAFETY: We just initialized the data pointed by `self`.
        unsafe { self.assume_init() }
    }
}

impl<'a, T: ?Sized> Uninit<'a, T> {
    /// Initializes `self` with the given constructor `f`.
    ///
    /// See also [`move_from`] and [`proj_fn`] for constructors other than plain closures.
    ///
    /// [`move_from`]: crate::move_from
    /// [`proj_fn`]: crate::proj_fn
    pub fn init_with<F>(self, f: F) -> Init<'a, T>
    where
        F: FnOnce(Uninit<'_, T>) -> Init<'_, T>,
    {
        f(self)
    }

    /// Assumes `self` has been initialized, returning an [`Init<T>`] token that guarantees
    /// this place has been initialized.
    ///
    /// # Safety
    /// The data pointed by `self` must have been initialized prior to this method call. See also
    /// [MaybeUninit::assume_init]'s docs.
    ///
    /// [MaybeUninit::assume_init]: core::mem::MaybeUninit::assume_init
    pub unsafe fn assume_init(mut self) -> Init<'a, T> {
        // SAFETY: `self.as_mut_ptr()` returns a valid pointer and
        // the caller ensures the data it points to has been initialized.
        unsafe { Init::from_raw(self.as_mut_ptr()) }
    }

    /// Creates a new `Uninit<T>` from a raw pointer.
    ///
    /// # Safety
    /// - `data` must be a valid and aligned pointer.
    /// - The `'a` lifetime must not be leaked to safe code.
    /// - In general `'a` must be different than the lifetime of other
    ///   `Uninit<'a, U>`s, unless they're part of the same allocation and
    ///   that allocation can be assumed initialized only when all the `Uninit<'a, U>`s
    ///   that form it are guaranteed to be initialized.
    pub unsafe fn from_ptr(data: *mut T) -> Uninit<'a, T> {
        // SAFETY: The caller ensures `data` is a valid pointer, thus non null.
        let data = unsafe { NonNull::new_unchecked(data) };
        Self {
            data,
            _invariant_lifetime: Invariant::default(),
        }
    }

    /// Returns a pointer to the data pointed by `self`
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_ptr()
    }
}

impl<'a, T> Uninit<'a, [T]> {
    /// Returns a reference to the underlying memory.
    pub fn as_maybeuninit_slice(&mut self) -> &mut [MaybeUninit<T>] {
        // TODO: Use `NonNull::as_uninit_slice_mut` when it gets stabilized.

        // Can't use `cast` because it requires `U` to be `Sized`
        let ptr = self.as_mut_ptr() as *mut [MaybeUninit<T>];
        // SAFETY: `self.as_mut_ptr()` returns a valid pointer and `MaybeUninit` doesn't
        // need to be initialized.
        unsafe { &mut *ptr }
    }

    /// Returns the length of the pointed slice.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        // TODO: Use `NonNull::len` when it gets stabilized.

        // Can't use `cast` because it requires `U` to be `Sized`.
        let ptr = self.data.as_ptr() as *const [MaybeUninit<T>];
        // SAFETY: `self.as_mut_ptr()` returns a valid pointer and `MaybeUninit` doesn't
        // need to be initialized.
        unsafe { (*ptr).len() }
    }
}

impl<'a, T: ?Sized> fmt::Debug for Uninit<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(type_name::<Self>())
    }
}

unsafe impl<'a, T: ?Sized + Send> Send for Uninit<'a, T> {}
unsafe impl<'a, T: ?Sized + Sync> Sync for Uninit<'a, T> {}
