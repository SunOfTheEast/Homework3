use std::ops::Deref;
use std::fmt;
use std::sync::Arc;

pub struct MyRc<T> {
    data : Arc<T>
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> MyRc<T> {
        MyRc {
            data : Arc::new(value)
        }
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        MyRc {
            data : self.data.clone()
        }
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self){

    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;


    fn deref(&self) -> &T {
        &self.data.deref()
    }
}
fn main() {
    let x = MyRc::new(1.5);
    {
        let y = MyRc::clone(&x);
        {
            let z = MyRc::clone(&x);
            println!("{}", x.strong_count());
            println!("{}", y.strong_count());
            println!("{}", z.strong_count());
            println!("{}", *x);
            println!("{}", *y);
            println!("{}", *z);
        }
        println!("{}", x.strong_count());
        println!("{}", y.strong_count());
        println!("{}", *x);
        println!("{}", *y);
    }
    println!("{}", x.strong_count());
    println!("{}", *x);
}
