mod intersperse;
mod intersperse2;

use crate::intersperse::{Intersperse as Intersperse1, IntersperseWith as Intersperse1With};
use crate::intersperse2::{Intersperse as Intersperse2, IntersperseWith as Intersperse2With};

pub trait IteratorExt: Iterator {
    #[inline]
    fn intersperse1(self, separator: Self::Item) -> Intersperse1<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Intersperse1::new(self, separator)
    }

    #[inline]
    fn intersperse1_with<G>(self, separator: G) -> Intersperse1With<Self, G>
    where
        Self: Sized,
        G: FnMut() -> Self::Item,
    {
        Intersperse1With::new(self, separator)
    }

    #[inline]
    fn intersperse2(self, separator: Self::Item) -> Intersperse2<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Intersperse2::new(self, separator)
    }

    #[inline]
    fn intersperse2_with<G>(self, separator: G) -> Intersperse2With<Self, G>
    where
        Self: Sized,
        G: FnMut() -> Self::Item,
    {
        Intersperse2With::new(self, separator)
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersperse() {
        let v: Vec<i32> = Vec::new();
        let mut it = v.iter().intersperse1(&0);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1];
        let mut it = v.iter().intersperse1(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2];
        let mut it = v.iter().intersperse1(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2, 3];
        let mut it = v.iter().intersperse1(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_intersperse_size_hint() {
        let iter = std::iter::empty::<i32>().intersperse1(0);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let xs = ["a", "", "b", "c"];
        let mut iter = xs.iter().map(|x| *x).intersperse1(", ");
        assert_eq!(iter.size_hint(), (7, Some(7)));

        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(", "));
        assert_eq!(iter.size_hint(), (5, Some(5)));

        assert_eq!([].iter().intersperse1(&()).size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_intersperse2() {
        let v: Vec<i32> = Vec::new();
        let mut it = v.iter().intersperse2(&0);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1];
        let mut it = v.iter().intersperse2(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2];
        let mut it = v.iter().intersperse2(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2, 3];
        let mut it = v.iter().intersperse2(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_intersperse_size_hint2() {
        let iter = std::iter::empty::<i32>().intersperse2(0);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let xs = ["a", "", "b", "c"];
        let mut iter = xs.iter().map(|x| *x).intersperse2(", ");
        assert_eq!(iter.size_hint(), (7, Some(7)));

        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(", "));
        assert_eq!(iter.size_hint(), (5, Some(5)));

        assert_eq!([].iter().intersperse2(&()).size_hint(), (0, Some(0)));
    }
}
