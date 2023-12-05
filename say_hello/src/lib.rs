#![allow(unused)]

pub fn pub_say_hello_outter() {
    println!("hello from pub_say_hello_outter")
}

fn say_hello_outter() {
    println!("hello from say_hello_outter")
}

pub mod pub_hi {
    pub fn pub_say_hi_inner() {
        println!("hi from pub_say_hi_inner")
    }
    fn say_hi_inner() {
        println!("hi from say_hi_inner")
    }
}

mod bye {
    pub fn pub_say_bye_inner() {
        println!("hi from pub_say_bye_inner")
    }
    fn say_bye_inner() {
        println!("hi from say_bye_inner")
    }
}
