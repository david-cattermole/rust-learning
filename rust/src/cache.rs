// Cache

type USize = usize;
type Key = u64;
type Value = String;
type ListType = std::collections::VecDeque<Key>;
type HashType = std::collections::HashMap<Key, (USize, Value)>;

pub struct Cache {
    capacity: USize,
    keys: ListType,
    key_to_value: HashType,
}

impl Cache {
    pub fn new() -> Cache {
        println!("New Cache");
        Cache {
            capacity: 0,
            keys: ListType::new(),
            key_to_value: HashType::new(),
        }
    }

    pub fn with_capacity(size: USize) -> Cache {
        println!("With Capacity {}", size);
        let mut c = Cache::new();
        Cache::set_capacity(&mut c, size);
        c
    }

    pub fn capacity(&self) -> USize {
        println!("Capacity");
        self.capacity
    }

    pub fn set_capacity(&mut self, size: USize) {
        println!("Set Capacity {}", size);
        self.capacity = size;
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        println!("Insert {}={}", key, value);
        if self.key_to_value.len() == self.capacity {
            self.evict_one();
        }
        let index: USize = self.keys.len();
        self.keys.push_back(key);
        self.key_to_value.insert(key, (index, value));
    }

    pub fn evict_one(&mut self) {
        println!("Evict One");
        let key = self.keys.pop_front();
        match key {
            Some(k) => {
                self.key_to_value.remove(&k);
            }
            None => panic!("Cannot evict, key does not exist."),
        };
    }

    pub fn remove(&mut self, key: Key) {
        println!("Remove {}", key);
        let ok = self.key_to_value.contains_key(&key);
        if ok == true {
            self.key_to_value.remove(&key);
            let pos = self.keys.iter().position(|k| k == &key);
            match pos {
                Some(idx) => {
                    self.keys.remove(idx);
                }
                None => {}
            };
        }
    }

    pub fn len(&self) -> USize {
        println!("Get Length");
        self.key_to_value.len()
    }

    // fn get() -> Value {}

    // fn get_size() -> u64 {}

    // fn remove(num_elements) {}

    // fn remove(percent) {}

    // fn clear() {}
}
