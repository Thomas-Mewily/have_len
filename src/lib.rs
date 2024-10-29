//! Define `is_empty(&self)` and `is_not_empty(&self)` 
//! for container that implement HaveLen, such as vector, array, string...
//! 
//! ```rust
//! use have_len::*;
//! 
//! assert_eq!([1, 2, 3].is_empty(), false);
//! assert_eq!([1, 2, 3].is_not_empty(), true);
//! 
//! let empty_array : [i32; 0] = []; 
//! assert_eq!(empty_array.is_empty(), true);
//! assert_eq!(empty_array.is_not_empty(), false);
//! 
//! assert_eq!("".is_empty(), true);
//! assert_eq!("hello".is_empty(), false);
//! 
//! assert_eq!("".is_not_empty(), false);
//! assert_eq!("hello".is_not_empty(), true);
//! 
//! assert_eq!("".is_empty(), !("".is_not_empty()));
//! ```

/// ```rust
/// use have_len::*;
/// 
/// assert_eq!([1, 2, 3].is_empty(), false);
/// assert_eq!([1, 2, 3].is_not_empty(), true);
/// 
/// let empty_array : [i32; 0] = []; 
/// assert_eq!(empty_array.is_empty(), true);
/// assert_eq!(empty_array.is_not_empty(), false);
/// ```
pub trait HaveLen
{
    fn len(&self) -> usize;

    /// True if the container contains at least one element.
    /// 
    /// ```rust
    /// use have_len::*;
    /// 
    /// assert_eq!([1, 2, 3].is_empty(), false);
    /// 
    /// let empty_array : [i32; 0] = []; 
    /// assert_eq!(empty_array.is_empty(), true);
    /// 
    /// assert_eq!("".is_empty(), true);
    /// assert_eq!("hello".is_empty(), false);
    /// 
    /// assert_eq!("".is_empty(), !("".is_not_empty()));
    /// ```
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// True if the container contains 0 element.
    /// 
    /// Will always be different than `is_empty()`.
    /// 
    ///
    /// ```rust
    /// use have_len::*;
    /// 
    /// assert_eq!([1, 2, 3].is_not_empty(), true);
    /// 
    /// let empty_array : [i32; 0] = []; 
    /// assert_eq!(empty_array.is_not_empty(), false);
    /// 
    /// assert_eq!("".is_not_empty(), false);
    /// assert_eq!("hello".is_not_empty(), true);
    /// 
    /// assert_eq!("".is_empty(), !("".is_not_empty()));
    /// ```
    fn is_not_empty(&self) -> bool { !self.is_empty() }
}

macro_rules! impl_length {
    ($($t:ident),*) => {
        $(
            impl<T> HaveLen for $t<T> { fn len(&self) -> usize { Self::len(&self) }}
        )*
    };
}

use std::collections::{VecDeque, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap, LinkedList};
impl_length!(Vec, VecDeque, HashSet, BinaryHeap, BTreeSet, LinkedList);

impl<K, V, S> HaveLen for HashMap<K, V, S>
{
    fn len(&self) -> usize { self.len() }
}
impl<K, V> HaveLen for BTreeMap<K, V>
{
    fn len(&self) -> usize { self.len() }
}
impl<T> HaveLen for [T]
{
    fn len(&self) -> usize { self.len() }
}

impl<T, const L : usize> HaveLen for [T; L]
{
    fn len(&self) -> usize { L }
}

impl HaveLen for String
{
    fn len(&self) -> usize { self.len() }
}

impl HaveLen for str
{
    fn len(&self) -> usize { self.len() }
}