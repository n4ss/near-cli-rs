//! Functions that cannot be implemented outside of this crate.
//!
//! Due to the [limited specification] of the encodings used by this crate,
//! some functions cannot be implemented compatibly. As a solution, this module
//! contains definitions that will work for all supported platforms, by taking
//! advantage of internal assumptions.
//!
//! These functions should only be passed bytes that can be given to
//! [`OsStrBytes::from_bytes`] without error. Only valid UTF-8 data or bytes
//! extracted using this crate are acceptable. Other sequences will not cause
//! safety issues, but they may result in panics or confusing results, so their
//! use is unsupported.
//!
//! [limited specification]: ../index.html#encoding
//! [`OsStrBytes::from_bytes`]: ../trait.OsStrBytes.html#tymethod.from_bytes

#![cfg_attr(os_str_bytes_docs_rs, doc(cfg(feature = "raw")))]

use super::imp::raw as imp;

/// Returns `true` if and only if the encoded bytes end with the given suffix.
///
/// The suffix is typed to not accept strings, because they do not make sense
/// to pass as the second argument to this function. While they will give the
/// expected result, [`slice::ends_with`] is more efficient to use in that
/// case.
///
/// # Panics
///
/// Either panics or returns an unspecified result if either sequence is
/// invalid.
///
/// # Examples
///
/// ```
/// use std::ffi::OsStr;
///
/// use os_str_bytes::OsStrBytes;
/// use os_str_bytes::raw;
///
/// let os_string = OsStr::new("bar");
/// let os_bytes = os_string.to_bytes();
/// assert!(raw::ends_with("foobar", &os_bytes));
/// ```
///
/// [`slice::ends_with`]: https://doc.rust-lang.org/std/primitive.slice.html#method.ends_with
#[inline]
#[must_use]
pub fn ends_with<TString>(string: &TString, suffix: &[u8]) -> bool
where
    TString: AsRef<[u8]> + ?Sized,
{
    imp::ends_with(string.as_ref(), suffix)
}

/// Returns `true` if and only if the encoded bytes start with the given
/// prefix.
///
/// The prefix is typed to not accept strings, because they do not make sense
/// to pass as the second argument to this function. While they will give the
/// expected result, [`slice::starts_with`] is more efficient to use in that
/// case.
///
/// # Panics
///
/// Either panics or returns an unspecified result if either sequence is
/// invalid.
///
/// # Examples
///
/// ```
/// use std::ffi::OsStr;
///
/// use os_str_bytes::OsStrBytes;
/// use os_str_bytes::raw;
///
/// let os_string = OsStr::new("foo");
/// let os_bytes = os_string.to_bytes();
/// assert!(raw::starts_with("foobar", &os_bytes));
/// ```
///
/// [`slice::starts_with`]: https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with
#[inline]
#[must_use]
pub fn starts_with<TString>(string: &TString, prefix: &[u8]) -> bool
where
    TString: AsRef<[u8]> + ?Sized,
{
    imp::starts_with(string.as_ref(), prefix)
}
