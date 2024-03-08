use multiple_packages::Asparagus;
use crate::garden::vegetables::Cabbage;

pub mod garden;

fn func() -> String {
    String::from("Hello this is someone")
}
fn main() {
    let plant = Asparagus{};
    println!("Plant 1 is {:?}", plant);
    let plant2 = Cabbage{};
    println!("Plant 2 is {:?}", plant2);
    let a = func();
    println!("{}", a);
}
