// Cache

type Key = u64;
type Value = String;
type ListType = std::collections::VecDeque<Key>;
type HashType = std::collections::HashMap<Key, Value>;


pub struct Cache {
    capacity: usize,
    keys: ListType,
    key_to_value: HashType,
}


impl Cache {
    pub fn new() -> Cache {
        println!("New Cache");
        let capacity = 0;
        let keys = ListType::new();
        let k_to_v = HashType::new();
        Cache {
            capacity: capacity,
            keys: keys,
            key_to_value: k_to_v,
        }
    }

    pub fn with_capacity(size: usize) -> Cache {
        println!("With Capacity {}", size);
        let mut c = Cache::new();
        Cache::set_capacity(&mut c, size);
        c
    }

    pub fn capacity(&self) -> usize {
        println!("Capacity");
        self.capacity
    }

    pub fn set_capacity(&mut self, size: usize) {
        println!("Set Capacity {}", size);
        self.capacity = size;
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        println!("Insert {}={}", key, value);
        self.keys.push_back(key);
        self.key_to_value.insert(key, value);
    }

    pub fn remove(&mut self, key: Key) {
        println!("Remove {}", key);
        let ok = self.key_to_value.contains_key(&key);
        if ok == true {
            self.key_to_value.remove(&key);

            let pos = self.keys.iter().position(|k | k == &key);
            match pos {
                Some(idx) => {
                    self.keys.remove(idx);
                },
                None => {}
            };
        }
    }

    pub fn len(&self) -> usize {
        self.key_to_value.len()
    }

    // fn get() -> Value {}

    // fn get_size() -> u64 {}

    // fn remove(num_elements) {}

    // fn remove(percent) {}

    // fn clear() {}
}
