#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // we needed to make v1_iter mutable:
    // : calling the next method on an iterator changes internal state that 
    // the iterator uses to keep track of where it is in the sequence.
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}