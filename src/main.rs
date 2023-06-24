mod uw;

fn main() {
    let mut my_ca = uw::UlamWarburtonCA::new(100, 100);
    my_ca.set(49, 49);
    my_ca.iterate(8);
    my_ca.show();
}
