mod lib;
use hash_set_map::HashSetMapBuilder;
use std::collections::hash_map::DefaultHasher;
fn main() {
    let mut hsm = HashSetMapBuilder::new();
    let mut hasher = DefaultHasher::new();
    let hash = hsm.insert(&mut hasher, "asd".to_string());
    println!("{hash:#?}");

    let mut hsm = HashSetMapBuilder::new();
    let mut hasher = &mut DefaultHasher::new();

    let hash = hsm.insert(hasher, "asd".to_string());
    println!("{hash:#?}");
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!(
        "{:#?}",
        hsm.insert(&mut DefaultHasher::new(), "asdddd".to_string())
    );
    println!("{:#?}", hsm.build());
}

/*
ideas
    - unify str vs String
    - unify &mut self vs Self
    - nested structs pass in super and return self of super?
    - add a macro to create a builder
    - figure out what derive and annotation do
    - put all the strings in an owned vec, in the name field store the index
    */
