

pub fn main() {
    println!("pierwszy");

    #[cfg(any(feature = "foo",feature = "foo1"))]{
    println!("start");}

    println!("drugi");
}