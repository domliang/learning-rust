mod m;

fn main() {
    println!("Hello, world!");
    let c : m::class::ClassName = m::class::ClassName::new(12);
    c.print();
    m::test::test();
    m::gi::guess_it();
}
