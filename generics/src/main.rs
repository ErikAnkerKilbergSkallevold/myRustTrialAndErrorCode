#![allow(unused)]
enum Option<T> {
    Some(T),
    None,
}
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl <X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }
    fn y(&self) -> &Y1 {
        &self.y
    }
    fn xy(&self) -> (&X1, &Y1) {
        (&self.x, &self.y)
    }
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> { //Only works if T is f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = smallest(&char_list);
    println!("The smallest char is {}", result);
    assert_eq!(result, 'a');

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 1.0 };
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("Integer x = {}", integer.x());
    println!("Float y = {}", &float.y());
    println!("Integer and float xy = {:?}", &int_and_float.xy());
    println!("Float distance from origin = {}", &float.distance_from_origin());
    println!("After mixup of p1 and p2, p3.x = {}, p3.y = {}", &p3.x, &p3.y);

    assert_eq!(integer.x, 5);
    assert_eq!(float.y, 4.0);
    assert_eq!(int_and_float.xy(), (&5, &1.0));
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'c');
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];

    for &item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
