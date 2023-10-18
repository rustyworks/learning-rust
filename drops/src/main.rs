use std::ops::Deref;

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("{}", *c);
    println!("{}", *d);
    println!("CustomSmartPointers created");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

impl Deref for CustomSmartPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
