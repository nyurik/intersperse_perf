use crate::intersperse::{MyIntersperse, MyIntersperseWith};

mod intersperse;

pub trait IteratorExt: Iterator {
    #[inline]
    fn my_intersperse(self, separator: Self::Item) -> MyIntersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        MyIntersperse::new(self, separator)
    }

    #[inline]
    fn my_intersperse_with<G>(self, separator: G) -> MyIntersperseWith<Self, G>
    where
        Self: Sized,
        G: FnMut() -> Self::Item,
    {
        MyIntersperseWith::new(self, separator)
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_intersperse() {
        let v: Vec<i32> = Vec::new();
        let mut it = v.iter().my_intersperse(&0);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1];
        let mut it = v.iter().my_intersperse(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2];
        let mut it = v.iter().my_intersperse(&0);
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&0));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let v = vec![1, 2, 3];
        let mut it = v.iter().my_intersperse(&0);
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
        let iter = std::iter::empty::<i32>().my_intersperse(0);
        assert_eq!(iter.size_hint(), (0, Some(0)));

        let xs = ["a", "", "b", "c"];
        let mut iter = xs.iter().map(|x| *x).my_intersperse(", ");
        assert_eq!(iter.size_hint(), (7, Some(7)));

        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some(", "));
        assert_eq!(iter.size_hint(), (5, Some(5)));

        assert_eq!([].iter().my_intersperse(&()).size_hint(), (0, Some(0)));
    }
}
