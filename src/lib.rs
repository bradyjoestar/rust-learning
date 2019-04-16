#[macro_use]
mod macros_demo;

// sequence is very important!
// If we want to use macro of macros_demo in module1
// we must put it below of macros_demo defintion.

pub mod mod_demo;

pub mod base_language_demo;

pub mod complex_language_demo;

pub mod ownership_demo;

pub mod closure_iterator_demo;

pub fn lib() {
    say_hello!();
}
