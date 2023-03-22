// 1. ----------------------------------------------------------------

use std::f64::consts::PI;

const RED_TIME: u32 = 20;
const GREEN_TIME: u32 = 10;
const YELLOW_TIME: u32 = 5;

#[derive(PartialEq)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Traffic {
    fn time(&self) -> u32;
}

impl Traffic for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => RED_TIME,
            TrafficLight::Green => GREEN_TIME,
            TrafficLight::Yellow => YELLOW_TIME,
        }
    }
}

// 2. ----------------------------------------------------------------
fn sum(list: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;				// 指明类型，否则下面的操作会把它变成u8
    for item in list {
        match sum.checked_add(*item) {
            Some(v) => sum = v,
            None => return None,
        }
    }
    Some(sum)
}

fn sum_2(list: &[u32]) -> Option<u32> {
    let mut iterator = list.iter();
    iterator.try_fold(0u32, |acc, &x| acc.checked_add(x))
}


// 3. ----------------------------------------------------------------
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64
}

struct Triangle {
    length: f64,
    height: f64,
}

struct Rectangle {
    length: f64,
    height: f64,
}

impl Circle {
    pub fn new(circle: Circle) -> Self {
        Self {
            radius: circle.radius
        }
    }
}

impl Triangle {
    pub fn new(triangle: Triangle) -> Self {
        Self {
            length: triangle.length,
            height: triangle.height
        }
    }
}

impl Rectangle {
    pub fn new(rectangle: Rectangle) -> Self {
        Self {
            length: rectangle.length,
            height: rectangle.height
        }
    }
}

fn print_area<T: Area>(t: &T) -> f64 {
    t.area()
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.height * self.length / 2.0
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

fn main() {

    // 1. --------------------------------------
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    assert_eq!(Traffic::time(&red), 20);
    assert_eq!(Traffic::time(&green), 10);
    assert_eq!(Traffic::time(&yellow), 5);

    // 2. --------------------------------------
    // 正常情况
    let list = vec![100, 200, 300];
    assert_eq!(sum(&list), Some(600));

    let list = vec![1, 2, 3 ];
    assert_eq!(sum_2(&list), Some(6));

    // 溢出时返回None
    let list = vec![2, u32::MAX-1];
    assert_eq!(sum(&list), None);

    let list = vec![2, u32::MAX-1];
    assert_eq!(sum_2(&list), None);

    // 3. --------------------------------------

    println!("{:?}", print_area(&Circle::new(Circle{ radius: 6.0 })));
    println!("{:?}", print_area(&Triangle::new(Triangle { length: 3.0, height: 4.0 })));
    println!("{:?}", print_area(&Rectangle::new(Rectangle { length: 3.0, height: 4.0 })));
}