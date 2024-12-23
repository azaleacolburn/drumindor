use crate::vector::Vector;
use core::{fmt::Debug, hash::Hash, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct HashMap<K, V>
where
    V: Sized + Clone + Debug,
    K: Sized + Clone + Debug + Hash,
{
    vec: Vector<V>,
    marker: PhantomData<K>,
}

impl<K, V> HashMap<K, V>
where
    V: Sized + Clone + Debug,
    K: Sized + Clone + Debug + Hash,
{
    pub fn new() -> HashMap<K, V> {
        let vec = Vector::<V>::new();
        HashMap {
            vec,
            marker: PhantomData,
        }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        let hashed_pos = hash(key);
        self.vec.set(value, hashed_pos);
    }

    pub fn get(&self, key: &str) -> V {
        let hashed_pos = hash(key);
        self.vec.get(hashed_pos)
    }
}

fn hash(value: &str) -> usize {
    let p = 31;
    let m = 1e9 as usize + 9;
    let mut hash_value: usize = 0;
    let mut p_pow: usize = 1;

    value.chars().into_iter().for_each(|c| {
        hash_value = (hash_value + (c as u8 - 'a' as u8 + 1) as usize * p_pow) % m;
        p_pow = (p_pow * p) % m;
    });

    hash_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let s: &str = "testing";
        let a: &str = "test";
        let h: &str = "g";
        let t: &str = "key";
        assert_eq!(hash(s), hash(s));
        assert_eq!(hash(a), hash(a));
        assert_eq!(hash(h), hash(h));
        assert_eq!(hash(t), hash(t));
    }

    #[test]
    fn basic_set_read() {
        let mut map = HashMap::<&str, u8>::new();
        map.insert("test", 1);
        map.insert("g", 2);
        map.insert("key", 3);

        assert_eq!(map.get("test"), 1);
        assert_eq!(map.get("g"), 2);
        assert_eq!(map.get("key"), 3);
    }
}
