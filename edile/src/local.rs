// TODO: Decide how to initialize local `Own`s

// #[macro_export]
// macro_rules! local {
//     (mut $val:ident $(: $ty:ty)?) => {
//         $crate::__helper(@LOCAL [mut $val] [$($ty)?] new_default)
//     };
//     ($val:ident $(: $ty:ty)?) => {
//         $crate::__helper(@LOCAL [$val] [$($ty)?] new_default)
//     };
//     (mut $val:ident $(: $ty:ty)? = new ( $($args:tt)* )) => {
//         $crate::__helper(@LOCAL [mut $val] [$($ty)?] new ($($args)*));
//     };
//     ($val:ident $(: $ty:ty)? = new ( $($args:tt)* )) => {
//         $crate::__helper(@LOCAL [$val] [$($ty)?] new ($($args)*));
//     };
//     (mut $val:ident $(: $ty:ty)? = with $uninit:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [mut $val] [$($ty)?] new_with |$uninit| $($body)*);
//     };
//     ($val:ident $(: $ty:ty)? = with $uninit:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [$val] [$($ty)?] new_with |$uninit| $($body)*);
//     };
//     (mut $val:ident $(: $ty:ty)? = with_proj $proj:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [mut $val] [$($ty)?] new_with_proj |_, $proj| $($body)*);
//     };
//     ($val:ident $(: $ty:ty)? = with_proj $proj:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [$val] [$($ty)?] new_with_proj |_, $proj| $($body)*);
//     };
// }

// #[macro_export]
// macro_rules! local {
//     ($(mut $($mut:tt)?)? $val:ident $(: $ty:ty)?) => {
//         $crate::__helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_default);
//     };
//     ($(mut $($mut:tt)?)? $val:ident $(: $ty:ty)? = new ( $($args:tt)* )) => {
//         $crate::__helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new ($($args)*));
//     };
//     ($(mut $($mut:tt)?)?  $val:ident $(: $ty:ty)? = with $uninit:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_with |$uninit| $($body)*);
//     };
//     ($(mut $($mut:tt)?)?  $val:ident $(: $ty:ty)? = with_proj $proj:pat => $($body:tt)*) => {
//         $crate::__helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_with_proj |_, $proj| $($body)*);
//     };
// }

// #[macro_export]
// macro_rules! local {
//     ($(mut $($mut:tt)?)? $val:ident $(: $ty:ty)? $(; $($($rest:tt)+)?)?) => {
//         $crate::__internal_helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_default);
//         $($crate::local!($($($rest)+)?))?;
//     };
//     ($(mut $($mut:tt)?)? $val:ident $(: $ty:ty)? = new ( $($args:tt)* ) $(; $($($rest:tt)+)?)?) => {
//         $crate::__internal_helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new ($($args)*));
//         $($crate::local!($($($rest)+)?))?;
//     };
//     ($(mut $($mut:tt)?)?  $val:ident $(: $ty:ty)? = with_proj $proj:pat => $body:expr $(; $($($rest:tt)+)?)?) => {
//         $crate::__internal_helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_with_proj |_, $proj| $body);
//         $($crate::local!($($($rest)+)?))?;
//     };
//     ($(mut $($mut:tt)?)?  $val:ident $(: $ty:ty)? = with $uninit:pat => $body:expr $(; $($($rest:tt)+)?)?) => {
//         $crate::__internal_helper(@LOCAL [$(mut $($mut)?)? $val] [$($ty)?] new_with |$uninit| $body);
//         $($crate::local!($($($rest)+)?))?;
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! __internal_helper {
//     (@LOCAL [$local:pat] [$($ty:ty)?] $method:ident $($args:tt)*) => {
//         let mut storage = ::core::mem::MaybeUninit::uninit();
//         let $local:pat $(: Own<$ty>)? = $crate::Own::$method(&mut storage, $($args)*);
//     };
// }
