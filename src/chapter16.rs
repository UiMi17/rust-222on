#[test]
fn test_01() {
    fn main() {
        let s1 = "hello";
        /* Fill in the blank */
        let s = format!("{}, world!", s1);
        assert_eq!(s, "hello, world!");
    }
    main();
}

#[test]
fn test_02() {
    fn main() {
        print!("hello world, ");
        println!("I am");
        println!("Sunface!");
    }
    main();
}