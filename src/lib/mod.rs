#[allow(dead_code)]
pub mod sha1_hash_map;
use core::panic;
use hex::encode;
use sha1::{Digest, Sha1};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{self, Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HashSetMap<T> {
    pub register: HashMap<Arc<u64>, Arc<T>>,
}

pub struct HashSetMapBuilder<T> {
    pub hash_register: HashSetMap<T>,
    pub hasher: Option<Arc<dyn Hasher>>,
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
    pub fn insert(mut self, hasher: &'a mut dyn Hasher, input: T) -> (u64, Option<Arc<T>>, Self) {
        if self.hasher.is_none() {
            panic!("HashSetMapBuilder::insert() -> hasher is None")
        };
        hasher.write(input.to_string().as_bytes());
        let key = hasher.finish();
        // print!("{}|", key);
        (
            key,
            self.hash_register
                .register
                .insert(Arc::new(key), Arc::new(input)),
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
    pub fn new_with_hasher(hasher: Arc<dyn Hasher>) -> Self {
        Self {
            hash_register: Default::default(),
            hasher: Some(hasher),
        }
    }
    pub fn new_with_capacity_and_hasher(capacity: usize, hasher: Arc<dyn Hasher>) -> Self {
        let mut hash_register = HashSetMap::default();
        hash_register.register = HashMap::with_capacity_and_hasher(capacity, Default::default());
        Self {
            hash_register,
            hasher: Some(hasher),
        }
    }
    pub fn build(self) -> HashMap<Arc<u64>, Arc<T>> {
        self.hash_register.register
    }

    pub fn with_hasher(mut self, hasher: Arc<dyn Hasher>) -> Self {
        self.hasher = Some(hasher);
        self
    }
    pub fn hasher(&mut self, hasher: Arc<dyn Hasher>) -> &mut Self {
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
            hasher: Some(Arc::new(DefaultHasher::new())),
        }
    }
}

#[allow(dead_code)]
impl Sha1Hasher
where
    Sha1Hasher: hash::Hasher,
{
    pub fn new() -> Self {
        Self { state: [0; 20] }
    }
    fn digest(&self, data: &[u8]) -> String {
        let mut hasher = Sha1Hasher::new();
        hasher.write(data);
        encode(hasher.state.as_ref())
        // hasher.finish()
    }
    pub fn write(&mut self, bytes: &[u8]) {
        // println!("{:?}", bytes);
        let mut hasher = Sha1::new();
        hasher.update(bytes);
        let encode = encode(hasher.finalize()).as_bytes().to_vec();
        // println!("{:#?}", encode);
        self.state = encode[0..20].try_into().unwrap();
    }

    pub fn finish(&self) -> u64 {
        let mut output: u64 = 0;
        for &byte in self.state[0..7].iter() {
            // print!("{}", byte);
            output = output << 8;
            output += byte as u64;
        }
        output
    }
}

#[allow(dead_code)]
impl hash::Hasher for Sha1Hasher {
    fn finish(&self) -> u64 {
        self.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.write(bytes);
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
    pub fn insert(mut self, input: String) -> (u64, Option<Arc<String>>, Self) {
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
    pub fn build(self) -> HashMap<Arc<u64>, Arc<String>> {
        self.hsm.build()
    }
}
