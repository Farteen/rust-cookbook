use std::fmt::Debug;

struct CustomSmartPointer<D>
where
D: Debug,
{
    data: D
}

impl<D> CustomSmartPointer<D>
where
D: Debug,
{
    fn new(data: D) -> Self {
        CustomSmartPointer {data}
    }
}

impl<D> Drop for CustomSmartPointer<D>
where
D: Debug,
{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer wih data `{:?}`", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer::new("A");
    let b = CustomSmartPointer::new("B");
    let c = CustomSmartPointer::new("C");
    let d = CustomSmartPointer::new("D");
    // The next line woould cause a compiler error, as desctructors cannot 
    // be explicitely called
    // c.drop();

    // The correct way to drop variables early is the following:
    std::mem::drop(c);
}