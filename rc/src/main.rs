use std::ops::Deref;
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
struct MyRc<T> {
    data: *mut T,
    count: *mut i32,
}
impl<T> MyRc<T> {
    fn new(data_ : T) -> MyRc<T> {
        MyRc {
            data : Box::into_raw(Box::new(data_)),
            count : Box::into_raw(Box::new(1)),
        }
    }
    
    fn strong_count(&self) -> i32{
        unsafe{
            *self.count
        }
    }

}
impl<T> Clone for MyRc<T> {
    fn clone(&self) -> MyRc<T> {
        unsafe {
            *self.count += 1;
        }
        MyRc {
            data: self.data,
            count: self.count,
        }
    }
}
impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.count -= 1;
            if *self.count == 0 {
                drop(Box::from_raw(self.data));
                drop(Box::from_raw(self.count));
            }
        }
    } 
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.data
        }
    }
}