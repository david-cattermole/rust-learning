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
    let cache_size = 1024;
    let mut c1 = cache::Cache::new();
    c1.set_capacity(cache_size);
    let mut c2 = cache::Cache::with_capacity(cache_size);
    assert_eq!(c1.capacity(), c2.capacity());

    // Insert
    c1.insert(k1, v1.clone());
    c1.insert(k2, v2.clone());
    let len1 = c1.len();
    println!("Size of Cache: {}", len1);
    assert_eq!(len1, 2);

    // Remove
    c1.remove(k1);
    c1.remove(k3);
    let len2 = c1.len();
    println!("Size of Cache: {}", len2);
    assert_eq!(len2, 1);

    // Insert
    c1.insert(k4, v4.clone());
    c2.insert(k3, v3.clone());
    c2.insert(k4, v4.clone());
    let len3 = c1.len();
    let len4 = c2.len();
    println!("Size of Cache: {}", len3);
    println!("Size of Cache: {}", len4);
    assert_eq!(len3, 2);
    assert_eq!(len4, 2);
}
