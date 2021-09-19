/// Utility macro to avoid repeating `impl FnOnce(Uninit<$ty>) -> Init<$ty>`.
#[macro_export]
macro_rules! ctor {
    ($ty:ty) => {
        impl $crate::__private::FnOnce($crate::Uninit<'_, $ty>) -> $crate::Init<'_, $ty>
    }
}
