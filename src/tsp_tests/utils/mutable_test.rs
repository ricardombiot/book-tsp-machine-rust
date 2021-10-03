/*use std::borrow::BorrowMut;



pub struct MutRef<'a,T>{
    unique_ref : &'a mut T
}

impl<'a,T> MutRef<'a,T> {
    pub fn new( unique_ref : &'a mut T) -> Self {
        Self{unique_ref}
    }
}

pub struct Mutable<'a,T> {
    value : T,
    unique_ref : Option<MutRef<'a,T>>,
    is_load: bool
}

impl<'a,T> Mutable<'a,T> {
    pub fn new(value: T) -> Self  { 
        let unique_ref = None;
        let obj = Self { value , unique_ref , is_load: false};
        return obj;
    } 

    pub fn load(&'a mut self){
        let my_ref : &'a mut T = &mut self.value;
        let unique_ref = MutRef::new(my_ref);
        self.unique_ref = Some(unique_ref);
        self.is_load = true;
    }

    /*
    pub fn get_ref(&'a mut self) -> & mut Option<MutRef<'a,T>>{
        if !self.is_load {
            self.load();
        }

        let option : &mut Option<MutRef<'a,T>> = &mut self.unique_ref;
        return option;
    }*/


}
*/

pub struct Layer<T> {
    value : T
}

impl<T> Layer<T> {
    pub fn new(value: T) -> Self { Self { value } }

    pub fn apply<R,F>(&self, func: F) -> R 
        where F : Fn(&T) -> R {

        return func(&self.value);
    }

    pub fn apply_mut<F>(&mut self, func: F)
        where F : Fn(&mut T) {
        func(&mut self.value);
    }

    pub fn get(&self) -> &T{
        return &self.value;
    }

}


#[test]
pub fn test_mutable(){

    let mut layer : Layer<i32> = Layer::new(10);
    assert_eq!(layer.apply(|x| x * 10),100);
    layer.apply_mut(|x| *x *= 10);
    assert_eq!(layer.get(),&100);
}