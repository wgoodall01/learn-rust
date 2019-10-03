use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;

pub const INITIAL_SIZE: usize = 13;
pub const MAX_LOAD_FACTOR: f32 = 0.67;

// Entry defines the possible states of an index in the backing table:
//  - None, if there has never been anything at that index.
//  - Removed, if there was an item there in the past, which has since been removed.
//  - Some, if there is currently an item there.
#[derive(Debug)]
enum Entry<K, V> {
    None,
    Removed,
    Some(K, V),
}

impl<K, V> Entry<K, V> {
    pub fn mut_value(&mut self) -> &mut V {
        match self {
            Entry::Some(_, value) => value,
            _ => panic!("unexpected non-value Entry found"),
        }
    }

    pub fn into_value(self) -> V {
        match self {
            Entry::Some(_, value) => value,
            _ => panic!("unexpected non-value Entry found"),
        }
    }
}

pub struct HashMap<K: Hash + Eq + Copy, V> {
    // Store the backing table on the heap
    table: Vec<Entry<K, V>>,

    // Store the number of Some{...} elements
    size: usize,
}

enum SearchResult {
    Found(usize), // key was found, it's at this index.
    Empty(usize), // key was not found, an empty space suitable for it at this index.
}

// TODO: Remove the Debug requirement
impl<K: Hash + Eq + Copy + fmt::Debug, V: fmt::Debug> HashMap<K, V> {
    pub fn new() -> HashMap<K, V> {
        HashMap {
            table: Self::allocate_table(INITIAL_SIZE),
            size: 0,
        }
    }

    pub fn new_capacity(capacity: usize) -> HashMap<K, V> {
        HashMap {
            table: Self::allocate_table(capacity),
            size: 0,
        }
    }

    // Allocates a backing table of the given size, on the heap, filling it
    // by default with Entry::None.
    fn allocate_table(size: usize) -> Vec<Entry<K, V>> {
        // New vector, setting each entry to Entry::None by default.
        let mut vec: Vec<Entry<K, V>> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(Entry::None);
        }
        vec
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.table.len()
    }

    fn search(&self, key: &K) -> SearchResult {
        // Calculate the hash of the key
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash: u64 = hasher.finish();

        let mut first_available: Option<usize> = None;

        for scan in 0..self.capacity() {
            let i = (hash as usize + scan) % self.capacity();
            let entry = &self.table[i];

            if let Entry::None | Entry::Removed = entry {
                first_available = first_available.or(Some(i));
            }

            match entry {
                // We've found the item at `key`. Return it.
                Entry::Some(k, _v) if k == key => return SearchResult::Found(i),

                // If we find an empty item, break.
                Entry::None => {
                    break;
                }

                // Ignore removed entries and other entries, if first_available is set.
                Entry::Some(..) | Entry::Removed => (),
            };
        }

        // Panic if we can't find an empty space.
        SearchResult::Empty(first_available.unwrap())
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // First, check the load factor, and grow if needed.
        let load_factor: f32 = self.len() as f32 / self.capacity() as f32;
        if load_factor > MAX_LOAD_FACTOR {
            self.grow(2 * self.capacity() + 1);
        }

        self.put_without_resize(key, value)
    }

    fn put_without_resize(&mut self, key: K, value: V) -> Option<V> {
        match self.search(&key) {
            SearchResult::Found(i) => {
                let new_entry = Entry::Some(key, value);

                // Swap out the entries in the map
                let old = mem::replace(&mut self.table[i], new_entry);

                // Return the old value
                Some(old.into_value())
            }

            SearchResult::Empty(i) => {
                // Add the new value, return None.
                self.table[i] = Entry::Some(key, value);
                self.size += 1;
                None
            }
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        match self.search(&key) {
            SearchResult::Found(i) => {
                let old = mem::replace(&mut self.table[i], Entry::Removed);
                self.size -= 1;
                Some(old.into_value())
            }
            SearchResult::Empty(_) => None,
        }
    }

    pub fn get(&mut self, key: K) -> Option<&mut V> {
        match self.search(&key) {
            SearchResult::Found(i) => Some(self.table[i].mut_value()),
            SearchResult::Empty(_) => None,
        }
    }

    pub fn contains(&self, key: K) -> bool {
        match self.search(&key) {
            SearchResult::Found(_) => true,
            SearchResult::Empty(_) => false,
        }
    }

    pub fn grow(&mut self, size: usize) {
        assert!(
            self.len() < size,
            "cannot resize to size smaller than len()"
        );

        // Allocate the new table, swap it into place, keep the old one.
        let old_table = mem::replace(&mut self.table, Self::allocate_table(size));
        self.size = 0;

        // Copy over all entries containing values by re-hashing and re-adding.
        for entry in old_table {
            if let Entry::Some(key, value) = entry {
                self.put_without_resize(key, value);
            }
        }
    }

    pub fn clear(&mut self) {
        self.table = Self::allocate_table(INITIAL_SIZE);
        self.size = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn create_map() {
        HashMap::<u32, u32>::new();
    }

    #[test]
    fn add_items() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 2);
        map.put(2, 4);
        map.put(3, 6);
    }

    #[test]
    fn get_items() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 2);
        map.put(2, 4);
        map.put(3, 6);
        assert_eq!(map.get(1), Some(&mut 2));
        assert_eq!(map.get(2), Some(&mut 4));
        assert_eq!(map.get(3), Some(&mut 6));
    }

    #[test]
    fn overwrite() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 10);
        map.put(1, 100);
        map.put(1, 2);
        assert_eq!(map.remove(1), Some(2));
        assert_eq!(map.get(1), None);
    }

    #[test]
    fn contains() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 2);
        map.put(2, 4);
        map.put(3, 6);
        assert!(map.contains(1));
        assert!(map.contains(2));
        assert!(map.contains(3));
        assert!(!map.contains(4));
        assert!(!map.contains(6));
        assert!(!map.contains(9));
    }

    #[test]
    fn remove_items() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 2);
        map.put(2, 4);
        map.put(3, 6);
        assert_eq!(map.remove(1), Some(2));
        assert_eq!(map.remove(2), Some(4));

        assert_eq!(map.get(3).unwrap(), &6);
        assert_eq!(map.get(1), None);
        assert_eq!(map.get(2), None);
        assert_eq!(map.get(100), None);
    }

    #[test]
    fn resize() {
        let mut map = HashMap::<u32, u32>::new();
        map.put(1, 2);
        map.put(2, 4);
        map.put(3, 6);
        map.grow(100);
        map.put(4, 8);
        map.put(5, 10);

        for x in 1..6 {
            assert_eq!(map.remove(x), Some(x * 2));
        }
    }
}
