//! Wordier versions of standard library operations.
//!
//! For cases where the normal names are terse to the point of being unclear.

#![no_std]

/// Returns whichever of `a` and `b` is larger. If they're equal, returns `b`.
///
/// This is equivalent to `max` from `core`, but avoids an issue where `max` is
/// interpreted in the conventional English sense of limiting something, as in
/// "speed: 10 (max 15)". This sense is the _inverse_ of `max`.
pub fn greater_of<T: Ord>(a: T, b: T) -> T {
    core::cmp::max(a, b)
}

/// Returns whichever of `a` and `b` is smaller. If they're equal, returns `b`.
///
/// This is equivalent to `min` from `core`, but avoids an issue where `min` is
/// interpreted in the conventional English sense of limiting something, as in
/// "speed: 10 (min 5)". This sense is the _inverse_ of `min`.
pub fn lesser_of<T: Ord>(a: T, b: T) -> T {
    core::cmp::min(a, b)
}
