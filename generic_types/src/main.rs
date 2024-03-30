use std::process::Output;
use std::ops::Mul;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest= &list[0];
    for i in 1..list.len(){
        if list[i] > (*largest){
            largest = &list[i];
        }
    }
    largest
}

#[derive(Clone, Copy)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }
    fn y(&self) -> &Y1 {
        &self.y
    }
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
pub trait PointDistance{
    fn origin_distance(&self) -> f64;
}

impl<X: Mul<Output = X> + Copy, Y: Mul<Output = Y> + Copy> Point<X, Y> {
    fn origin_distance(&self) -> f64
    where
        X: Into<f64>,
        Y: Into<f64>,
    {
        let x_square: f64 = (self.x).into().mul((self.x).into());
        let y_square: f64 = (self.y).into().mul((self.y).into());
        let sum_val = x_square + y_square;
        sum_val.sqrt()
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let vector = vec![20, 10, 10, 15, 19, 1, 202, 21, 204];
    let result = largest(&vector);

    let vector = vec![12.4, 213.21, 213.2, 123.231, 21.5123, 342.43];
    let result2 = largest(&vector);

    let vector = vec!['a', 'c', 'z', 'e', 'u', 'r', 'k'];
    let result3 = largest(&vector);

    println!("{result}, {result2}, {result3}");

    let p = Point{x: 20, y: 10};
    println!("p: x is {}, y is {}", p.x(), p.y());

    let f = Point{x: 20.5, y: 21.4};
    println!("f: x is {}, y is {}", f.x, f.y);

    let mix = f.mixup(p.clone());
    println!("mix: x is {}, y is {}", mix.x, mix.y);


    println!("Point distance is {}", mix.origin_distance());
    println!("Point distance 2 is {}", p.origin_distance());

    let novel = String::from("Call me hello world. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let mut i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("first_sentence {}", first_sentence);
    println!("{0}", i.part);

    {
        let novel = String::from("This is a great dicitonary");
        let second_sentence = novel.split('d').next().expect("Could not find a 'd'");
        // i.part = second_sentence; 
    }
    // println!("{0}", i.part); // This will throw error as novel is dropped before
    let novel = novel;
    // println!("{0}", i.part) // won't work as novel was moved in the previous line!
    // note: .split() method only takes the reference from novel, so i.part is not affected by
    // first_sentence and is only affected by novel
    //
}
