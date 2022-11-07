use task1::LRUCache;

#[test]
fn simple_cache() {
    let mut result = LRUCache::<i32, i32>::new(3);
    result.get_or_compute(1, || { 7 });
    // panic is not called, so it really caches
    result.get_or_compute(1, || { panic!() });

    assert_eq!(result.get(&1).unwrap().clone(), 7);
    assert_eq!(result.size(), 1);
    assert_eq!(result.max_size(), 3);
}

#[test]
fn drop_not_used() {
    let mut result = LRUCache::<i32, i32>::new(3);
    result.get_or_compute(1, || { 5 });
    result.get_or_compute(2, || { 6 });
    result.get_or_compute(3, || { 7 });
    result.get_or_compute(4, || { 8 });

    assert!(result.get(&1).is_none());
    assert_eq!(result.get(&2).unwrap(), &6);
    assert_eq!(result.get(&3).unwrap(), &7);
    assert_eq!(result.get(&4).unwrap(), &8);
    assert_eq!(result.size(), 3);
    assert_eq!(result.max_size(), 3);
}

#[test]
#[should_panic]
fn cant_create_zero_size() {
    LRUCache::<i32, i32>::new(0);
}
