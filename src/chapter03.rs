#[test]
fn test_01() {
    // Fix the error below with the least amount of modification to the code
    fn main() {
        let x: i32 = 5; // Uninitialized but used, ERROR !

        assert_eq!(x, 5);
        println!("Success!");
    }
    main()
}

#[test]
fn test_02() {
    // Fill the blanks in the code to make it compile
    fn main() {
        let mut x = 1;
        x += 2;

        assert_eq!(x, 3);
        println!("Success!");
    }
    main()
}

#[test]
fn test_03() {
    // Fix the error below with the least amount of modification
    fn main() {
        let x: i32 = 10;
        let y: i32 = 5;
        {
            println!("The value of x is {} and value of y is {}", x, y);
        }
        println!("The value of x is {} and value of y is {}", x, y);
    }
    main()
}

#[test]
fn test_04() {
    // Fix the error with the use of define_x
    let x = define_x();
    fn main(x: &str) {
        println!("{}, world", x);
    }

    fn define_x() -> &'static str {
        "hello"
    }
    main(x)
}

#[test]
fn test_05() {
    // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    fn main() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // Prints "42".
    }
    main()
}

#[test]
fn test_06() {
    // Remove a line in the code to make it compile
    fn main() {
        let mut x: i32 = 1;
        x = 7;
        // Shadowing and re-binding
        let _x = x;


        let _y = 4;
        // Shadowing
        let _y = "I can also be bound to text!";

        println!("Success!");
    }
    main()
}

#[test]
fn test_07() {
    fn main() {
        let _x = 1;
    }
    // Warning: unused variable: `x`
    main()
}

#[test]
fn test_08() {
    // Fix the error below with the least amount of modification
    fn main() {
        let (mut x, y) = (1, 2);
        x += 2;

        assert_eq!(x, 3);
        assert_eq!(y, 2);

        println!("Success!");
    }
    main()
}

#[test]
fn test_09() {
    // Fix the error below with the least amount of modification
    fn main() {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];
        // Fill the blank to make the code work
        assert_eq!([x,y], [3, 2]);

        println!("Success!");
    }
    main()
}