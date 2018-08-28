#[derive(Clone)]
pub struct VecMap<K, V> {
    vec: Vec<(K, V)>,
}

impl<K: PartialEq, V> VecMap<K, V> {
    #[inline]
    pub fn with_capacity(cap: usize) -> VecMap<K, V> {
        VecMap {
            vec: Vec::with_capacity(cap)
        }
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        // not using entry or find_mut because of borrowck
        for entry in &mut self.vec {
            if key == entry.0 {
                *entry = (key, value);
                return;
            }
        }
        self.vec.push((key, value));
    }

    #[inline]
    pub fn append(&mut self, key: K, value: V) {
        self.vec.push((key, value));
    }

    #[inline]
    pub fn get<K2: PartialEq<K> + ?Sized>(&self, key: &K2) -> Option<&V> {
        self.find(key).map(|entry| &entry.1)
    }

    #[inline]
    pub fn len(&self) -> usize { self.vec.len() }

    #[inline]
    pub fn iter(&self) -> ::std::slice::Iter<(K, V)> {
        self.vec.iter()
    }

    #[inline]
    pub fn remove_all<K2: PartialEq<K> + ?Sized>(&mut self, key: &K2) {
        let len = self.vec.len();
        for i in (0..len).rev() {
            if key == &self.vec[i].0 {
                self.vec.remove(i);
            }
        }
    }

    #[inline]
    fn find<K2: PartialEq<K> + ?Sized>(&self, key: &K2) -> Option<&(K, V)> {
        for entry in &self.vec {
            if key == &entry.0 {
                return Some(entry);
            }
        }
        None
    }
}
