use std::marker::PhantomData;
use std::collections::BTreeMap;
use entity::*;
use component::*;

use toml::*;

pub trait Deserializable {
    fn deserialize(&Value) -> Self;
}

pub trait Deserializers {
    fn visit(&self, &Entity, toml : &BTreeMap<String, Value>);

    fn deserialize(&self, e : &Entity, source : &str) {
        let toml = Parser::new(source).parse().unwrap();
        self.visit(e, &toml);
    }
}

pub struct DeserializersStorage<T : Component + Deserializable,
                            U : Deserializers> {
    name : String,
    marker : PhantomData<T>,
    next : U
}


impl<T : Component + Deserializable,
     U : Deserializers> Deserializers for DeserializersStorage<T, U> {
    fn visit(&self, entity : &Entity, source : &BTreeMap<String, Value>) {
        if let Some(value) = source.get(&self.name) {
            let c : T = Deserializable::deserialize(value);
            entity.add_component(c);
        }
        self.next.visit(entity, source);
    }
}

impl Deserializers for () {
    fn visit(&self, _ : &Entity, _ : &BTreeMap<String, Value>) {
    }
}

impl<T : Component + Deserializable> DeserializersStorage<T, ()> {
    pub fn new(name : String, _ : PhantomData<T>) ->
        DeserializersStorage<T, ()> {
            DeserializersStorage {
                name   : name,
                marker : PhantomData,
                next   : ()
            }
        }
}

impl<T : Component + Deserializable,
     U : Deserializers> DeserializersStorage<T, U> {
    pub fn add<T1 : Component + Deserializable>(self, name : String) ->
        DeserializersStorage<T1, DeserializersStorage<T, U>> {
            DeserializersStorage {
                name : name,
                marker : PhantomData,
                next : self
            }
        }
}

#[macro_export]
macro_rules! deserializers {
    ($t:ty) => {
        DeserializersStorage::new(stringify!($t).to_string(), PhantomData::<$t>)
    };
    ($t1:ty, $($t:ty),+) => {
        {
            let deserializers = DeserializersStorage::new(stringify!($t1).to_string(), PhantomData::<$t1>);
            $(
                let deserializers = deserializers.add::<$t>(stringify!($t).to_string());
            )+
            let deserializers : Box<Deserializers> = Box::new(deserializers);
            deserializers
        }
    };
}

