use std::collections::HashMap;
use std::hash::Hash;

pub trait DictInmutableWapper<K,V> where K : Hash + Eq + Clone, V: Clone{
    
    fn dict(&self) -> & HashMap<K, V> ;
    fn dict_mut(&mut self) -> &mut HashMap<K, V> ;

    fn insert(&mut self, key : &K, value : &V){
        if !self.contains_key(key){
            self.update(key, value);
        }
    }

    fn update(&mut self, key : &K, value : &V){
        self.dict_mut().insert(key.clone(), value.clone());
    }

    fn get(&self, key : &K) -> Option<V>{
        let result = match self.dict().get(key) {
            None => None,
            Some(value) => Some(value.clone())
        };

        return result;
    }

    fn contains_key(&self, key : &K) -> bool {
        self.dict().contains_key(key)
    }

    fn remove(&mut self, key : &K){
        if self.contains_key(&key){
            self.dict_mut().remove(key);
        }
    }

    fn is_empty(&self)-> bool {
        self.dict().is_empty()
    }
}


#[derive(Clone)]
pub struct InmutableDict<K,V> where K : Hash + Eq + Clone, V: Clone {
    dict : HashMap<K,V>,
}

impl<K,V> InmutableDict<K,V> where 
K : Hash + Eq + Clone, 
V: Clone {

    pub fn new() -> Self {
        let dict : HashMap<K, V> = HashMap::new();
        Self{dict}
    }

    pub fn to_list(&self) -> Vec<(K,V)> {
        let mut iterable_list : Vec<(K,V)> = Vec::new();
        for (k, v ) in self.dict.iter() {
            iterable_list.push((k.clone(),v.clone()));
        }

        return iterable_list;
    }

    pub fn to_list_keys(&self) -> Vec<K> {
        let mut iterable_list : Vec<K> = Vec::new();
        for (k, v ) in self.dict.iter() {
            iterable_list.push(k.clone());
        }

        return iterable_list;
    }

    pub fn for_each<F>(&self, mut func : F) where F : FnMut(&K, &V) {
        for (key, value) in self.to_list().iter() {
            func(key, value)
        }
    }

    pub fn union(&mut self, other : &InmutableDict<K,V>) {
        for (key, value) in other.dict.iter() {
            self.insert(key, value);
        }
    }
}



impl<K,V> DictInmutableWapper<K,V> for InmutableDict<K,V> where 
    K : Hash + Eq + Clone, 
    V: Clone  {

    fn dict(&self) -> & HashMap<K, V> {
        &self.dict
    }

    fn dict_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.dict
    }

}


pub trait InmutableDictCommons<K, V> where 
K : Hash + Eq + Clone, 
V: Clone {

    fn dict(&self) -> & InmutableDict<K, V> ;
    fn dict_mut(&mut self) -> &mut InmutableDict<K, V> ;


    fn put(&mut self, key : &K, value : &V) {
        self.dict_mut().update(key, value);
    } 

    fn have(&self, key : &K) -> bool {
        self.dict().contains_key(key)
    }

    fn delete(&mut self, key : &K){
        self.dict_mut().remove(key);
    }

    fn get(&self, key : &K) -> Option<V> {
        return self.dict().get(key)
    }

    fn is_empty(&self)-> bool {
        self.dict().is_empty()
    }

    fn to_list(&self) -> Vec<(K, V)> {
        return self.dict().to_list();
    }

    fn union(&mut self, other : &dyn InmutableDictCommons<K, V>){
        self.dict_mut().union(&other.dict());
    }
}