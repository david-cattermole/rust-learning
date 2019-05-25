mod cache;

fn main() {
    let k1 = 123;
    let k2 = 124;
    let k3 = 125;
    let k4 = 126;

    let cache_size = 1024;
    let mut x = cache::Cache::new();
    x.set_capacity(cache_size);
    let v1 = String::from("My String 001");
    let v2 = String::from("My String 002");
    x.insert(k1, v1);
    x.insert(k2, v2);

    let len1 = x.len();
    println!("Size of Cache: {}", len1);
    assert_eq!(len1, 2);

    x.remove(k1);
    x.remove(k3);

    let len2 = x.len();
    println!("Size of Cache: {}", len2);
    assert_eq!(len2, 1);
    
}
