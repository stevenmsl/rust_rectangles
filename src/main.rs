#[derive(Debug)] //opt in for printing out debugging info
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
      - &self is short for self: &Self
      - Self is an alias for the type
        that the impl block is for
      - we use &self because we just
        want to read the data and not
        to take the ownership
      - area is a method; a method is a
        kind of associated function - it
        let you specify the behavior that
        instances of your structs have
    */
    fn area(&self) -> u32 {
        self.width * self.height //expression; no semicolon required
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /*
      - the method can have the same name as the field
    */
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    /*
      - :? tells println! to use an output format
        called Debug
      - :#? makes it look a bit prettier
    */
    if rect1.width() {
        println!("rect 1 is {:#?} has area of {} ", rect1, rect1.area());
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
}
