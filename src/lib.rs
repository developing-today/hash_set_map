use core::panic;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HashSetMap<T> {
    register: HashMap<Rc<u64>, Rc<T>>,
}

pub struct HashSetMapBuilder<T> {
    hash_register: HashSetMap<T>,
    hasher: Option<Rc<dyn Hasher>>,
}
impl<'a, T> HashSetMapBuilder<T>
where
    T: Hash + Eq + Clone + Default + ToString,
    HashSetMapBuilder<T>: Default,
{
    pub fn insert(&mut self, hasher: &'a mut dyn Hasher, input: T) -> Option<Rc<T>> {
        if self.hasher.is_none() {
            panic!("HashSetMapBuilder::insert() -> hasher is None")
        };
        hasher.write(input.to_string().as_bytes());
        let key = hasher.finish();
        self.hash_register
            .register
            .insert(Rc::new(key), Rc::new(input))
    }
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        let mut hash_register = HashSetMap::default();
        hash_register.register = HashMap::with_capacity(capacity);
        Self {
            hash_register,
            hasher: None,
        }
    }
    pub fn new_with_hasher(hasher: Rc<dyn Hasher>) -> Self {
        Self {
            hash_register: Default::default(),
            hasher: Some(hasher),
        }
    }

    pub fn new_with_capacity_and_hasher(capacity: usize, hasher: Rc<dyn Hasher>) -> Self {
        let mut hash_register = HashSetMap::default();
        hash_register.register = HashMap::with_capacity_and_hasher(capacity, Default::default());
        Self {
            hash_register,
            hasher: Some(hasher),
        }
    }
    pub fn build(self) -> HashMap<Rc<u64>, Rc<T>> {
        self.hash_register.register
    }

    pub fn with_hasher(mut self, hasher: Rc<dyn Hasher>) -> Self {
        self.hasher = Some(hasher);
        self
    }
    pub fn hasher(&mut self, hasher: Rc<dyn Hasher>) -> &mut Self {
        self.hasher = Some(hasher);
        self
    }

    pub fn len(&self) -> usize {
        self.hash_register.register.len()
    }
    pub fn is_empty(&self) -> bool {
        self.hash_register.register.is_empty()
    }
    pub fn clear(&mut self) {
        self.hash_register.register.clear();
    }
}
impl<T> Default for HashSetMapBuilder<T>
where
    T: Hash + Eq + Clone + Default + ToString,
{
    fn default() -> Self {
        Self {
            hash_register: HashSetMap {
                register: HashMap::new(),
            },
            hasher: Some(Rc::new(DefaultHasher::new())),
        }
    }
}
