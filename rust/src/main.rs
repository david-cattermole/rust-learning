mod cache;

fn main() {
    let v1 = String::from("My String 001");
    let v2 = String::from("My String 002");
    let v3 = String::from("My String 003");
    let v4 = String::from("My String 004");
    let k1 = 123;
    let k2 = 124;
    let k3 = 125;
    let k4 = 126;

    // Create Cache
    let cache_size = 3;
    let mut c_a = cache::Cache::new();
    c_a.set_capacity(cache_size);
    let mut c_b = cache::Cache::with_capacity(cache_size);
    assert_eq!(c_a.capacity(), c_b.capacity());
    println!("============================");

    // Insert
    c_a.insert(k1, v1.clone());
    c_a.insert(k2, v2.clone());
    let len1 = c_a.len();
    println!("Size of CacheA: {}", len1);
    assert_eq!(len1, 2);
    println!("============================");

    // Remove
    c_a.remove(k1);
    c_a.remove(k3);
    let len2 = c_a.len();
    println!("Size of CacheA: {}", len2);
    assert_eq!(len2, 1);
    println!("============================");

    // Insert
    c_a.insert(k4, v4.clone());
    c_b.insert(k3, v3.clone());
    c_b.insert(k4, v4.clone());
    let len3 = c_a.len();
    let len4 = c_b.len();
    println!("Size of CacheA: {}", len3);
    println!("Size of CacheB: {}", len4);
    assert_eq!(len3, 2);
    assert_eq!(len4, 2);
    println!("============================");

    // Insert all keys to both caches, test the eviction.
    c_a.insert(k1, v1.clone());
    c_a.insert(k2, v2.clone());
    c_a.insert(k3, v3.clone());
    c_a.insert(k4, v4.clone());
    let len5 = c_a.len();
    println!("Size of CacheA: {}", len5);

    c_b.insert(k1, v1.clone());
    c_b.insert(k2, v2.clone());
    c_b.insert(k3, v3.clone());
    c_b.insert(k4, v4.clone());
    let len6 = c_b.len();
    println!("Size of CacheB: {}", len6);
    assert_eq!(len5, 3);
    assert_eq!(len6, 3);
}
