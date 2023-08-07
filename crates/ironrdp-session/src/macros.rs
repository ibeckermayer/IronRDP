/// Creates a `SessionError` with `General` kind
///
/// Shorthand for
/// ```rust
/// <crate::SessionError as crate::SessionErrorExt>::general(context)
/// ```
#[macro_export]
macro_rules! general_err {
    ( $context:expr $(,)? ) => {{
        <$crate::SessionError as $crate::SessionErrorExt>::general($context)
    }};
}

/// Creates a `SessionError` with `Reason` kind
///
/// Shorthand for
/// ```rust
/// <crate::SessionError as crate::SessionErrorExt>::reason(context, reason)
/// ```
#[macro_export]
macro_rules! reason_err {
    ( $context:expr, $($arg:tt)* ) => {{
        <$crate::SessionError as $crate::SessionErrorExt>::reason($context, format!($($arg)*))
    }};
}

/// Creates a `SessionError` with `Custom` kind and a source error attached to it
///
/// Shorthand for
/// ```rust
/// <crate::SessionError as crate::SessionErrorExt>::custom(context, source)
/// ```
#[macro_export]
macro_rules! custom_err {
    ( $context:expr, $source:expr $(,)? ) => {{
        <$crate::SessionError as $crate::SessionErrorExt>::custom($context, $source)
    }};
}

/// A helper function to assert that a type implements all traits in a list.
///
/// Ripped directly from the
/// [static_assertions crate v1.1.0](https://docs.rs/static_assertions/1.1.0/src/static_assertions/assert_impl.rs.html#113-121)
#[macro_export]
macro_rules! assert_impl_all {
    ($type:ty: $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            // Only callable when `$type` implements all traits in `$($trait)+`.
            fn assert_impl_all<T: ?Sized $(+ $trait)+>() {}
            assert_impl_all::<$type>();
        };
    };
}
