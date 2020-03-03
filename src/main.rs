#[macro_use]
extern crate custom_derive;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct Switzerland;