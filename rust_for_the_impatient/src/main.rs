use std::cmp::min;
fn main() {
    let y;
    //foobar(y); //Crashes
    let x: i32 = 42;
    let _ = 42;
    y = 42;
    foobar(y);

    enum Option<T> {
        Some(T),
        None,
    }

    impl<T> Option<T> {
        fn unwrap(self) -> T {
            match self {
                Option::Some(x) => x,
                Option::None => panic!("None"),
            }
        }
    }

    struct Pair<T> {
        x: T,
        y: T,
    }

    let p1 = Pair { x: 1, y: 2 };
    let p2 = Pair { x: true, y: false};

    struct Number {
        odd: bool,
        value: i32,
    }

    impl Number {
        fn is_positive(&self) -> bool {
            self.value > 0
        }
        fn is_negative(&self) -> bool {
            self.value < 0
        }
    }

    let minus_two = Number {
        odd: true,
        value: -2,
    };

    println!("{}", minus_two.is_positive());

    let least = min(7, 1);
    println!("The least is {}", least);

    {
        let x = {
            let y = 42;
            let z = 23;
            y + z
        };
        println!("{}", x);
    }

    let pair = ('a', 17);
    pair.0;
    pair.1;

    let tuple = (1, "hello", 4.5);
    let (some_char, some_int) = ('a', 42);
    assert!(some_char == 'a');
    assert!(some_int == 42);


    for i in vec![52, 49, 32] {
        println!("I like the number {}", i);
    }
}

fn foobar(n: i32) {
    println!("{}", n);
}
