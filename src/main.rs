fn main() {
    // let name_length = 
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    fn new(name: &str) -> Self {
        NameLength{
            name: name.to_owned(),
            length: name.len(),
        }
    }

    fn print(&self) {
        println!("The name '{}' is '{}' characters long", self.name, self.length);
    }
}

use std::borrow::Cow;
struct NameLengthLT<'a> {
    name: Cow<'a, str>,
    length: usize,
}

impl<'a> NameLengthLT<'a> {
    fn new<S>(name: S) -> Self
    where 
        S: Into<Cow<'a, str>>, {
            let name: Cow<'a, str> = name.into();
            /**
             * watch out the diffs below
            NameLengthLT {
                length: name.len(),
                name,
            }
             */
            NameLengthLT {
                length: name.len(),
                name,
            }
        }

    fn print(&self) {
        println!("The name '{}‘ is '{}' characters long", self.name, self.length);
    }
}