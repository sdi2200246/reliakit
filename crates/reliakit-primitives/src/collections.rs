use crate::{PrimitiveError, PrimitiveResult};
use alloc::vec::Vec;
use core::ops::Deref;

/// Owned vector guaranteed to contain at least one element.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    /// Creates a `NonEmptyVec` from a `Vec`. Returns `Empty` if the vec is empty.
    pub fn new(vec: Vec<T>) -> PrimitiveResult<Self> {
        if vec.is_empty() {
            return Err(PrimitiveError::Empty);
        }
        Ok(Self(vec))
    }

    /// Creates a `NonEmptyVec` containing a single item. Never fails.
    pub fn from_one(item: T) -> Self {
        Self(alloc::vec![item])
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always returns `false`.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Returns a reference to the first element.
    pub fn first(&self) -> &T {
        &self.0[0]
    }

    /// Returns a reference to the last element.
    pub fn last(&self) -> &T {
        &self.0[self.0.len() - 1]
    }

    /// Appends an element to the end.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Returns the inner slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Consumes the wrapper and returns the inner `Vec`.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = PrimitiveError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        Self::new(vec)
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(value: NonEmptyVec<T>) -> Self {
        value.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::NonEmptyVec;
    use crate::PrimitiveError;

    #[test]
    fn rejects_empty_vec() {
        assert_eq!(
            NonEmptyVec::<i32>::new(alloc::vec![]).unwrap_err(),
            PrimitiveError::Empty
        );
    }

    #[test]
    fn accepts_non_empty_vec() {
        let v = NonEmptyVec::new(alloc::vec![1, 2, 3]).unwrap();
        assert_eq!(v.len(), 3);
        assert!(!v.is_empty());
    }

    #[test]
    fn from_one() {
        let v = NonEmptyVec::from_one(42);
        assert_eq!(v.first(), &42);
        assert_eq!(v.last(), &42);
    }

    #[test]
    fn first_and_last() {
        let v = NonEmptyVec::new(alloc::vec![10, 20, 30]).unwrap();
        assert_eq!(v.first(), &10);
        assert_eq!(v.last(), &30);
    }

    #[test]
    fn push_increases_len() {
        let mut v = NonEmptyVec::from_one(1);
        v.push(2);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn deref_to_slice() {
        let v = NonEmptyVec::new(alloc::vec![1, 2, 3]).unwrap();
        assert_eq!(&v[..], &[1, 2, 3]);
    }

    #[test]
    fn into_inner() {
        let v = NonEmptyVec::new(alloc::vec![1, 2]).unwrap();
        assert_eq!(v.into_inner(), alloc::vec![1, 2]);
    }

    #[test]
    fn try_from_vec() {
        assert!(NonEmptyVec::<i32>::try_from(alloc::vec![]).is_err());
        assert!(NonEmptyVec::try_from(alloc::vec![1]).is_ok());
    }

    #[test]
    fn from_into_vec() {
        let v = NonEmptyVec::from_one(99);
        let inner: alloc::vec::Vec<i32> = alloc::vec::Vec::from(v);
        assert_eq!(inner, alloc::vec![99]);
    }

    #[test]
    fn iter() {
        let v = NonEmptyVec::new(alloc::vec![1, 2, 3]).unwrap();
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
    }
}
