#[test]
fn test_01() {
    fn main_01() {
        let x = String::from("hello, world");
        let y = x.clone();
        println!("{},{}",x,y);
    }

    fn main_02() {
        let x = &String::from("hello, world");
        let y = x;
        println!("{},{}",x,y);
    }

    fn main_03() {
        let x = String::from("hello, world");
        let y = x.as_str();
        println!("{},{}",x,y);
    }
    main_01();
    main_02();
    main_03();
}

#[test]
fn test_02() {
    // Don't modify code in main!
    fn main() {
        let s1 = String::from("hello, world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
    main();
}
#[test]
fn test_03() {
    fn main() {
        let s = give_ownership();
        println!("{}", s);
    }

    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("hello, world");
        // convert String to Vec
        let _s = s.as_bytes();
        s
    }
    main();
}
#[test]
fn test_04() {
    fn main() {
        let s = String::from("hello, world");
        print_str(&s);
        println!("{}", s);
    }
    fn print_str(s: &String)  {
        println!("{}",s)
    }
    main();
}
#[test]
fn test_05() {
    fn main() {
        let x = (1, 2, (), "hello");
        let y = x;
        println!("{:?}, {:?}", x, y);
    }
    main();
}
#[test]
fn test_06() {
    fn main() {
        let s = String::from("hello, ");

        // modify this line only !
        let mut s1 = s;

        s1.push_str("world")
    }
    main();
}
#[test]
fn test_07() {
    fn main() {
        let x = Box::new(5);

        let mut y = Box::new(3);       // implement this line, don't change other lines!

        *y = 4;

        assert_eq!(*x, 5);
    }
    main();
}

#[test]
fn test_08() {
    fn main() {
        let t = (String::from("hello"), String::from("world"));

        let _s = t.0;

        // modify this line only, don't use `_s`
        println!("{:?}", t.1);
    }
    main();
}

#[test]
fn test_09() {
    fn main() {
        let t = (String::from("hello"), String::from("world"));

        // fill the blanks
        let (s1, s2) = t.clone();

        println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    }
    main();
}
