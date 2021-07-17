use m::{class::ClassName, gi::guess_it, test::test};

mod m;

fn main() {
    let mut c: ClassName = ClassName::new(12);
    let s: String = String::from("rs");
    c.print();
    c.hi(s.clone());
    print!("{:#?}", c);
    test();
    guess_it();
}
