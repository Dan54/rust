use core::iter::*;

use super::*;

#[test]
fn test_fuse_nth() {
    let xs = [0, 1, 2];
    let mut it = xs.iter();

    assert_eq!(it.len(), 3);
    assert_eq!(it.nth(2), Some(&2));
    assert_eq!(it.len(), 0);
    assert_eq!(it.nth(2), None);
    assert_eq!(it.len(), 0);
}

#[test]
fn test_fuse_last() {
    let xs = [0, 1, 2];
    let it = xs.iter();

    assert_eq!(it.len(), 3);
    assert_eq!(it.last(), Some(&2));
}

#[test]
fn test_fuse_count() {
    let xs = [0, 1, 2];
    let it = xs.iter();

    assert_eq!(it.len(), 3);
    assert_eq!(it.count(), 3);
    // Can't check len now because count consumes.
}

#[test]
fn test_fuse_fold() {
    let xs = [0, 1, 2];
    let it = xs.iter(); // `FusedIterator`
    let i = it.fuse().fold(0, |i, &x| {
        assert_eq!(x, xs[i]);
        i + 1
    });
    assert_eq!(i, xs.len());

    let it = xs.iter(); // `FusedIterator`
    let i = it.fuse().rfold(xs.len(), |i, &x| {
        assert_eq!(x, xs[i - 1]);
        i - 1
    });
    assert_eq!(i, 0);

    let it = xs.iter().scan((), |_, &x| Some(x)); // `!FusedIterator`
    let i = it.fuse().fold(0, |i, x| {
        assert_eq!(x, xs[i]);
        i + 1
    });
    assert_eq!(i, xs.len());
}

#[test]
fn test_fuse() {
    let mut it = 0..3;
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.len(), 1);
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.len(), 0);
}

#[test]
fn test_peekable_fuse() {
    // test with I: FusedIterator
    test_peekable_iterator((0..5).fuse());

    // test with unfused iterator
    let mut iter = CycleIter::new(&[0, 1]).fuse();
    assert!(iter.peek_with(|x| x == Some(&&0)));
    assert_eq!(iter.next(), Some(&0));
    assert!(iter.peek_with(|x| x == Some(&&1)));
    assert_eq!(iter.next(), Some(&1));
    assert!(iter.peek_with(|x| x == None));
    assert_eq!(iter.next(), None);
    assert!(iter.peek_with(|x| x == None));
    assert_eq!(iter.next(), None);
}
