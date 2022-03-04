#[allow(dead_code)]
use core::panic;
use hex::encode;
use sha1::{Digest, Sha1};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HashSetMap<T> {
    pub register: HashMap<Rc<String>, Rc<T>>,
}

pub struct HashSetMapBuilder<T> {
    pub hash_register: HashSetMap<T>,
    pub hasher: Option<Rc<dyn Hasher>>,
}

pub struct Sha1Hasher {
    pub state: [u8; 20],
}
pub struct HsmWithSha1Hasher {
    pub hsm: HashSetMapBuilder<String>,
    pub hasher: Sha1Hasher,
}

#[allow(dead_code)]
impl<'a, T> HashSetMapBuilder<T>
where
    T: Hash + Eq + Clone + Default + ToString,
    HashSetMapBuilder<T>: Default,
{
    pub fn insert(
        mut self,
        hasher: &mut Sha1Hasher,
        input: T,
    ) -> (Rc<String>, Option<Rc<T>>, Self) {
        if self.hasher.is_none() {
            panic!("HashSetMapBuilder::insert() -> hasher is None")
        };
        hasher.write(input.to_string().as_bytes());
        let key = Rc::new(hasher.finish());
        // print!("{}|", key);
        (
            key.clone(),
            self.hash_register.register.insert(key, Rc::new(input)),
            self,
        )
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
    pub fn build(self) -> HashMap<Rc<String>, Rc<T>> {
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

#[allow(dead_code)]
impl Sha1Hasher {
    pub fn new() -> Self {
        Self { state: [0; 20] }
    }
    fn digest(&self, data: &[u8]) -> String {
        let mut hasher = Sha1Hasher::new();
        hasher.write(data);
        encode(hasher.state.as_ref())
    }
    pub fn write(&mut self, bytes: &[u8]) {
        // println!("{:?}", bytes);
        let mut hasher = Sha1::new();
        hasher.update(bytes);
        let encode = hasher.finalize();
        // println!("{:#?}", encode);
        self.state = encode[0..20].try_into().unwrap();
    }

    pub fn finish(&self) -> String {
        encode(self.state.as_ref())
    }
}

#[allow(dead_code)]
impl HsmWithSha1Hasher {
    pub fn new() -> Self {
        Self {
            hsm: HashSetMapBuilder::new(),
            hasher: Sha1Hasher::new(),
        }
    }
    pub fn insert(mut self, input: String) -> (Rc<String>, Option<Rc<String>>, Self) {
        let key;
        let previous;
        (key, previous, self.hsm) = self.hsm.insert(&mut self.hasher, input);
        (key, previous, self)
    }
    pub fn insert_blind(mut self, input: String) -> Self {
        let _key;
        let _previous;
        (_key, _previous, self) = self.insert(input);
        self
    }
    pub fn build(self) -> HashMap<Rc<String>, Rc<String>> {
        self.hsm.build()
    }
}
