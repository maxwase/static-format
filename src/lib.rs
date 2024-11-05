#![cfg_attr(feature = "no-std", no_std)]
#![cfg_attr(
    any(feature = "ascii", feature = "ascii-extended"),
    allow(confusable_idents, uncommon_codepoints, non_camel_case_types)
)]

use core::fmt;
use core::marker::PhantomData;

#[cfg(feature = "ascii")]
pub mod ascii;

/// A statically typed character that can be concatenated with others.
#[derive(Debug, Default)]
pub struct Char<const C: char, T = Nil>(PhantomData<T>);

impl<T: fmt::Display + Default, const C: char> fmt::Display for Char<C, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{C}{}", T::default())
    }
}

/// End of a word.
#[derive(Debug, Default)]
pub struct Nil;

impl fmt::Display for Nil {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// A modifier that concatenates 2 chains.
#[derive(Debug, Default)]
pub struct Chain<H, T>(H, T);

impl<H: fmt::Display, T: fmt::Display> fmt::Display for Chain<H, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)?;
        fmt::Display::fmt(&self.1, f)
    }
}

/// A modifier that repeats the character or sequence it wraps.
#[derive(Debug, Default)]
pub struct Repeat<C, const N: usize = 1>(PhantomData<C>);

impl<C, const N: usize> fmt::Display for Repeat<C, N>
where
    C: fmt::Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..N {
            fmt::Display::fmt(&C::default(), f)?;
        }
        Ok(())
    }
}

/// A modifier that changes [fmt::Display] and [fmt::Debug] implementations.
#[derive(Default)]
pub struct DisplaySwapDebug<T>(T);

impl<T: fmt::Display> fmt::Debug for DisplaySwapDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T: fmt::Debug> fmt::Display for DisplaySwapDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// A macro to declare a type that represents a word.
///
/// # Examples
/// ```rust
/// use static_format::decl_word_type;
///
/// type Word = decl_word_type!('W', 'o', 'r', 'd');
///
/// # fn main() {
/// let word = Word::default();
/// assert_eq!(word.to_string(), "Word");
/// # }
/// ```
#[macro_export]
macro_rules! decl_word_type {
    ($($c:expr),+ $(,)?) => {
        $crate::decl_word_type_impl!($($c),+)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! decl_word_type_impl {
    ($c:expr) => {
        $crate::Char<$c>
    };
    ($c:expr, $($rest:expr),+) => {
        $crate::Char<$c, decl_word_type_impl!($($rest),+)>
    };
}

#[cfg(all(test, feature = "ascii"))]
mod tests {
    use ascii::*;

    use super::*;

    #[test]
    fn word() {
        type Word = decl_word_type!('W', 'o', 'r', 'd');

        let word = Word::default();
        assert_eq!(word.to_string(), "Word");

        let word = W::<o<r<d>>>::default();
        assert_eq!(word.to_string(), "Word");
    }

    #[test]
    fn repeat() {
        let word: W<Chain<Repeat<o, 3>, r<d>>> = Default::default();
        assert_eq!(word.to_string(), "Wooord");

        let repeated_wo: W<Repeat<W<o>, 3>> = Default::default();
        assert_eq!(repeated_wo.to_string(), "WWoWoWo");
    }

    #[test]
    fn new_line() {
        let word: W<Nl<o<r<d>>>> = Default::default();
        assert_eq!(word.to_string(), "W\nord");
    }

    #[test]
    fn display_swap_debug() {
        let word: W<Chain<DisplaySwapDebug<o>, r<d>>> = Default::default();
        assert_eq!(word.to_string(), "WChar(PhantomData<static_format::Nil>)rd");

        assert_eq!(
            DisplaySwapDebug::<DisplaySwapDebug<o>>::default().to_string(),
            "o"
        );
    }
}
