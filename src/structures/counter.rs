use std::collections::HashMap;

pub struct Counter<T: Eq + std::hash::Hash> {
    map: HashMap<T, u32>,
}

#[allow(dead_code)]
impl<T: Eq + std::hash::Hash> Counter<T> {
    pub fn new() -> Self {
        Counter {
            map: HashMap::new(),
        }
    }

    pub fn increment(&mut self, t: T) {
        *self.map.entry(t).or_insert(0) += 1;
    }

    pub fn decrement(&mut self, t: T) {
        match self.map.entry(t) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.get_mut() -= 1;
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {}
        }
    }

    pub fn get(&self, t: T) -> u32 {
        *self.map.get(&t).unwrap_or(&0)
    }
}

impl<T: Eq + std::hash::Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter: Counter<T> = Counter::new();
        for item in iter {
            counter.increment(item);
        }
        counter
    }
}
