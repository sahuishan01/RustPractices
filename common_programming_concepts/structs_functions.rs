use std::fmt;

#[derive(Clone)]
struct Point{
    x: f64,
    y: f64,
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "\nX point is {}, Y point is {}", self.x, self.y)
    }
}

struct Rectangle{
    top_left : Point,
    bottom_right : Point,
}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f, "\nTop left point is {},\nBottom right point is {}", self.top_left, self.bottom_right)
    }
}

fn rect_area(rect: &Rectangle)-> f64{
    (rect.top_left.x - rect.bottom_right.x) * (rect.top_left.y - rect.bottom_right.y)
}

fn square(pair: (Point, f64)) -> Rectangle{
    let x = pair.0.x + pair.1;
    let y = pair.0.y + pair.1;
    Rectangle{
        top_left: pair.0,
        bottom_right: Point{x: x, y: y},
    }
}


fn main(){
    // creating two points
    let point1 = Point{x: 0.3, y: 0.4};
    let point2 = Point{x: 0.5, y: 0.6};
    println!("points are: {} {}", point1, point2);

    let rect1 = Rectangle{top_left: point1.clone(), bottom_right: point2};
    println!("{}", rect1);

    // creating square
    let rect2 = square((point1, 10.0));
    println!("square is: {}\n and area is: {} units", rect2, rect_area(&rect2));
}