// Deref and DerefMut are traits that are used to overload the dereference operator *.
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let str = Box::new(String::from("Hello World!"));
    // Below line will work because &Box<String> is Deref to &String, and &String is Deref to &str
    // Deref only works for those types that implement Deref and DerefMut traits
    print(&str);

    let str_2: MySmartPointer<Box<String>> = MySmartPointer::new(Box::new(String::from("Hello World!")));
    // &MySmartPointer<Box<String>> -> Deref -> &Box<String> -> Deref -> &String -> Deref -> &str
    // So, below line will work
    print(&str_2);

    // When use * operator, it will dereference the value of the pointer
    // _str_3 is of type &MySmartPointer<Box<String>>
    let _str_3 = &(str_2);
    // _str_3 is of type &Box<String>
    let _str_3 = &(*str_2);
    // str_3 is of type &String
    let _str_3 = &(**str_2);
    // _str_3 is of type &str
    let _str_3 = &(***str_2);

    // DerefMut Example
    let str_4 = String::from("Hello World!");
    let mut str_4 = MySmartPointer::new(Box::new(str_4));
    print_mut(&mut str_4);
}

fn print(s: &str) {
    println!("{}", s);
}

fn print_mut(s: &mut str) {
    println!("{}", s);
}