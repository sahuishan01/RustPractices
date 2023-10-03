use std::fmt;

#[derive(Clone)]
// defining custom struct for point
struct Point{
    x: f64,
    y: f64,
}

// implementing display for custom struct Point
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "\tX point is {}, Y point is {}", self.x, self.y)
    }
}

// custom struct for rectangle
struct Rectangle{
    top_left : Point,
    bottom_right : Point,
}

// custom display for struct rectangle
impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f, "\n\tTop left point is: {},\n\tBottom right point is: {}", self.top_left, self.bottom_right)
    }
}

// defining additional functions for struct Rectangle
impl Rectangle{

    // To calculate area of rectangle
    fn area(&self) -> f64{
        ((self.top_left.x - self.bottom_right.x) * (self.top_left.y - self.bottom_right.y)).abs()
    }

    // To calculate it's width
    fn width(&self) -> f64{
        (self.bottom_right.x - self.top_left.x).abs()
    }

    // To calculate it's height
    fn height(&self) -> f64{
        (self.top_left.y - self.bottom_right.y).abs()
    }
}

// function for creating square using Rectangle struct
fn square(pair: (Point, f64)) -> Rectangle{
    let x = pair.0.x - pair.1;
    let y = pair.0.y - pair.1;
    Rectangle{
        top_left: pair.0,
        bottom_right: Point{x, y },
    }
}


fn main(){
    // creating two points
    let point1 = Point{x: 0.3, y: 0.4};
    let point2 = Point{x: 0.5, y: 0.6};
    println!("CREATING CUSTOM STRUCT OF POINTS");
    println!("points are:{} {}", point1, point2);
    println!();

    let rect1 = Rectangle{top_left: point1.clone(), bottom_right: point2};
    println!("CREATING CUSTOM STRUCT OF RECTANGLE USING POINTS STRUCT");
    println!("{}", rect1);
    println!();

    // creating square
    println!("CREATING SQUARE WITH FROM RECTANGLE USING TOP LEFT POINT");
    let rect2 = square((point1, 10.0));
    println!("square is: {}\nWidth is {} and Height is {}\nAnd area is: {} units", rect2, rect2.width(), rect2.height(), rect2.area());
}