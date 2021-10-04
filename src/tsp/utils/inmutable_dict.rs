use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

pub trait DictInmutableWapper<K,V> where K : Hash + Eq + Clone, V: Clone{
    
    fn dict(&self) -> & HashMap<K, V> ;
    fn dict_mut(&mut self) -> &mut HashMap<K, V> ;
    //fn dict_mut_life<'user>(&'user mut self) -> &'user mut HashMap<K, V> ;

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut HashMap<K, V> {
        self.dict_mut()
    }

    fn dict_life<'user>(&'user self) -> &'user HashMap<K, V> {
        self.dict()
    }

    fn insert(&mut self, key : &K, value : &V){
        if !self.contains_key(key){
            self.update_borrows(key, value);
        }
    }

    fn update_borrows(&mut self, key : &K, value : &V){
        self.dict_mut().insert(key.clone(), value.clone());
    }

    fn update(&mut self, key : K, value : V){
        self.dict_mut().insert(key, value);
    }

    fn get<'user>(&'user self, key : &'user K) -> Option<&'user V>{
        let result : Option<&'user V> = match self.dict_life().get(key) {
            None => None,
            Some(value) => Some(value)
        };

        return result;
    }

    fn get_mut<'user>(&'user mut self, key : &'user K) -> Option<&'user mut V>{
        let dict : &'user mut HashMap<K, V> = self.dict_mut_life();
        let value_mutable : Option<&'user mut V> = dict.get_mut(key);
        return value_mutable;
    }


    fn contains_key(&self, key : &K) -> bool {
        self.dict().contains_key(key)
    }

    fn remove(&mut self, key : &K){
        if self.contains_key(&key){
            self.dict_mut().remove(key);
        }
    }

    fn pop(&mut self, key : &K) -> Option<V> { 
        self.dict_mut().remove(key)
    }

    fn is_empty(&self)-> bool {
        self.dict().is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct InmutableDict<K,V> where K : Hash + Eq + Clone, V: Clone {
    dict : HashMap<K,V>,
}

impl<K,V> InmutableDict<K,V> where 
K : Hash + Eq + Clone + Debug, 
V: Clone + Debug{

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
        for (k, _v ) in self.dict.iter() {
            iterable_list.push(k.clone());
        }

        return iterable_list;
    }

    pub fn for_each<F>(&self, mut func : F) where F : FnMut(&K, &V) {
        for (key, value) in self.to_list().iter() {
            func(key, value)
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

    fn dict_mut_life<'user>(&'user mut self) -> &'user mut HashMap<K, V> {
        &mut self.dict
    }

}


pub trait InmutableDictCommons<K, V> where 
K : Hash + Eq + Clone + Debug, 
V: Clone + Debug {

    fn dict(&self) -> & InmutableDict<K, V> ;
    fn dict_mut(&mut self) -> &mut InmutableDict<K, V> ;
    fn dict_mut_life<'user>(&'user mut self) -> &'user mut InmutableDict<K, V>;

    fn join_item(original : &mut V, join : &V);

    fn join(&mut self, other: &Self){
        self.join_table(other.dict())
    }

    fn join_table(&mut self, other : &InmutableDict<K,V>) {
        for (join_key, join_value) in other.dict.iter() {
            if self.have(join_key){
                let value_original = self.get_mut(join_key).unwrap();

                Self::join_item(value_original, join_value);
            }else{
                self.dict_mut().update(join_key.clone(), join_value.clone());
            }
        }
    }

    fn put_borrows(&mut self, key : &K, value : &V) {
        self.dict_mut().update_borrows(key, value);
    } 

    fn put(&mut self, key : K, value : V) {
        self.dict_mut().update(key, value);
    }

    fn put_ifnew_borrows(&mut self, key : &K, value : &V) {
        if !self.have(key) {
            self.dict_mut().update_borrows(key, value);
        }
    } 

    fn put_ifnew(&mut self, key : K, value : V) {
        if !self.have(&key) {
            self.dict_mut().update(key, value);
        }
    }

    fn have(&self, key : &K) -> bool {
        self.dict().contains_key(key)
    }

    fn delete(&mut self, key : &K){
        self.dict_mut().remove(key);
    }

    fn get<'user>(&'user self, key : &'user K) -> Option<&'user V> {
        return self.dict().get(key)
    }

    fn get_mut<'user>(&'user mut self, key : &'user K) -> Option<&'user mut V> {
        return self.dict_mut_life().get_mut(key);
    } 

    fn pop(&mut self, key : &K) -> Option<V> {
        return self.dict_mut().pop(key);
    }

    fn is_empty(&self)-> bool {
        self.dict().is_empty()
    }

    fn to_list(&self) -> Vec<(K, V)> {
        return self.dict().to_list();
    }

    fn to_list_keys(&self) -> Vec<K> {
        return self.dict().to_list_keys();
    }

    fn apply<R,F>(&self, key: &K, func: F) -> Option<R> 
        where F : Fn(&V) -> R {
            match self.dict().get(key) {
                None => None,
                Some(item) => {
                    let result = func(&item);
                    Some(result)
                }
            }
    }

    fn apply_mut<F>(&mut self, key: &K, func: F) 
        where F : Fn(&mut V)  {
            match self.dict_mut().get_mut(key) {
                None => (),
                Some(item) => {
                    func(item);
                }  
            }
    }

    /*
    fn union(&mut self, other : &Self){
        self.dict_mut().union(&other.dict());
    }*/
}


