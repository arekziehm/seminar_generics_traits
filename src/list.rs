use std::fmt::Display;

use crate::List::*;

enum List<T> {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(T, Box<List<T>>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        Nil
    }

    fn prepend(self, elem: T) -> List<T> {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(), // equivalent to Cons(.., ref tail) => 1 + tail.len()?
            Nil => 0
        }
    }


}

impl<T: Display> List<T>{
    fn stringify(&self) -> String {
        match self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}



fn main() {
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);


    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let mut text_list = List::new();
    text_list = text_list.prepend("text1");
    text_list = text_list.prepend("text2");
    text_list = text_list.prepend("text3");
    println!("text linked list has length: {}", list.len());
    println!("{}", text_list.stringify());

    // let mut n_list = List::new();
    // n_list = n_list.prepend(());
    // println!("{}", n_list.stringify());


}
