struct Point<T, U> {
    x: T,
    y: U,
}

struct PointString {
    //    x: &str, This will cause compile error
//    y:&str,  This will cause compile error
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

pub fn generic_struct() {
    println!("{}", "------------generic string start-------------------");
    let string_and_string = Point {
        x: String::from("generic"),
        y: String::from("demo"),
    };
    println!("{}", string_and_string.x);
    println!("{}", string_and_string.y);

    let mut x = String::from("demo");
    let mut y = String::from("generic");
    let mut xpointer = &mut x;
    let mut ypointer = &mut y;

    let mut string_string = Point {
        x: xpointer,
        y: ypointer,
    };

    println!("{}", string_string.x);

    string_string.x.push_str("_generic");

    //cannot borrow `x` as immutable because it is also borrowed as mutable
    //println!("x value is :{}",x);
}

pub fn generic_method() {
    println!("{}", "------------generic method start-------------------");
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("p3.x= {}", p3.x());
}
