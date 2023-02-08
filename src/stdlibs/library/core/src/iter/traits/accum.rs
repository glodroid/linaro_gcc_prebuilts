use crate::iter;
use crate::num::Wrapping;

/// Trait to represent types that can be created by summing up an iterator.
///
/// This trait is used to implement [`Iterator::sum()`]. Types which implement
/// this trait can be generated by using the [`sum()`] method on an iterator.
/// Like [`FromIterator`], this trait should rarely be called directly.
///
/// [`sum()`]: Iterator::sum
/// [`FromIterator`]: iter::FromIterator
#[stable(feature = "iter_arith_traits", since = "1.12.0")]
pub trait Sum<A = Self>: Sized {
    /// Method which takes an iterator and generates `Self` from the elements by
    /// "summing up" the items.
    #[stable(feature = "iter_arith_traits", since = "1.12.0")]
    fn sum<I: Iterator<Item = A>>(iter: I) -> Self;
}

/// Trait to represent types that can be created by multiplying elements of an
/// iterator.
///
/// This trait is used to implement [`Iterator::product()`]. Types which implement
/// this trait can be generated by using the [`product()`] method on an iterator.
/// Like [`FromIterator`], this trait should rarely be called directly.
///
/// [`product()`]: Iterator::product
/// [`FromIterator`]: iter::FromIterator
#[stable(feature = "iter_arith_traits", since = "1.12.0")]
pub trait Product<A = Self>: Sized {
    /// Method which takes an iterator and generates `Self` from the elements by
    /// multiplying the items.
    #[stable(feature = "iter_arith_traits", since = "1.12.0")]
    fn product<I: Iterator<Item = A>>(iter: I) -> Self;
}

macro_rules! integer_sum_product {
    (@impls $zero:expr, $one:expr, #[$attr:meta], $($a:ty)*) => ($(
        #[$attr]
        impl Sum for $a {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    $zero,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a + b,
                )
            }
        }

        #[$attr]
        impl Product for $a {
            fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    $one,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a * b,
                )
            }
        }

        #[$attr]
        impl<'a> Sum<&'a $a> for $a {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    $zero,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a + b,
                )
            }
        }

        #[$attr]
        impl<'a> Product<&'a $a> for $a {
            fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    $one,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a * b,
                )
            }
        }
    )*);
    ($($a:ty)*) => (
        integer_sum_product!(@impls 0, 1,
                #[stable(feature = "iter_arith_traits", since = "1.12.0")],
                $($a)*);
        integer_sum_product!(@impls Wrapping(0), Wrapping(1),
                #[stable(feature = "wrapping_iter_arith", since = "1.14.0")],
                $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum_product {
    ($($a:ident)*) => ($(
        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl Sum for $a {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    0.0,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a + b,
                )
            }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl Product for $a {
            fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    1.0,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a * b,
                )
            }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl<'a> Sum<&'a $a> for $a {
            fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    0.0,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a + b,
                )
            }
        }

        #[stable(feature = "iter_arith_traits", since = "1.12.0")]
        impl<'a> Product<&'a $a> for $a {
            fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    1.0,
                    #[rustc_inherit_overflow_checks]
                    |a, b| a * b,
                )
            }
        }
    )*)
}

integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum_product! { f32 f64 }

#[stable(feature = "iter_arith_traits_result", since = "1.16.0")]
impl<T, U, E> Sum<Result<U, E>> for Result<T, E>
where
    T: Sum<U>,
{
    /// Takes each element in the [`Iterator`]: if it is an [`Err`], no further
    /// elements are taken, and the [`Err`] is returned. Should no [`Err`]
    /// occur, the sum of all elements is returned.
    ///
    /// # Examples
    ///
    /// This sums up every integer in a vector, rejecting the sum if a negative
    /// element is encountered:
    ///
    /// ```
    /// let v = vec![1, 2];
    /// let res: Result<i32, &'static str> = v.iter().map(|&x: &i32|
    ///     if x < 0 { Err("Negative element found") }
    ///     else { Ok(x) }
    /// ).sum();
    /// assert_eq!(res, Ok(3));
    /// ```
    fn sum<I>(iter: I) -> Result<T, E>
    where
        I: Iterator<Item = Result<U, E>>,
    {
        iter::try_process(iter, |i| i.sum())
    }
}

#[stable(feature = "iter_arith_traits_result", since = "1.16.0")]
impl<T, U, E> Product<Result<U, E>> for Result<T, E>
where
    T: Product<U>,
{
    /// Takes each element in the [`Iterator`]: if it is an [`Err`], no further
    /// elements are taken, and the [`Err`] is returned. Should no [`Err`]
    /// occur, the product of all elements is returned.
    fn product<I>(iter: I) -> Result<T, E>
    where
        I: Iterator<Item = Result<U, E>>,
    {
        iter::try_process(iter, |i| i.product())
    }
}

#[stable(feature = "iter_arith_traits_option", since = "1.37.0")]
impl<T, U> Sum<Option<U>> for Option<T>
where
    T: Sum<U>,
{
    /// Takes each element in the [`Iterator`]: if it is a [`None`], no further
    /// elements are taken, and the [`None`] is returned. Should no [`None`]
    /// occur, the sum of all elements is returned.
    ///
    /// # Examples
    ///
    /// This sums up the position of the character 'a' in a vector of strings,
    /// if a word did not have the character 'a' the operation returns `None`:
    ///
    /// ```
    /// let words = vec!["have", "a", "great", "day"];
    /// let total: Option<usize> = words.iter().map(|w| w.find('a')).sum();
    /// assert_eq!(total, Some(5));
    /// ```
    fn sum<I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = Option<U>>,
    {
        iter::try_process(iter, |i| i.sum())
    }
}

#[stable(feature = "iter_arith_traits_option", since = "1.37.0")]
impl<T, U> Product<Option<U>> for Option<T>
where
    T: Product<U>,
{
    /// Takes each element in the [`Iterator`]: if it is a [`None`], no further
    /// elements are taken, and the [`None`] is returned. Should no [`None`]
    /// occur, the product of all elements is returned.
    fn product<I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = Option<U>>,
    {
        iter::try_process(iter, |i| i.product())
    }
}
