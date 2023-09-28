// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

struct Point<T: std::ops::AddAssign, U: std::ops::AddAssign> {
    x: T,
    y: U,
}

impl<T: std::ops::AddAssign, U: std::ops::AddAssign> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn move_to(self: &mut Self, x: T, y: U) -> () {
        self.x += x;
        self.y += y;
    }
}

impl AppendBar for String {
    fn append_bar(self: Self) -> Self {
        // String::from(format!("{}{}", self.as_str(), "Bar"))
        format!("{self}Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
