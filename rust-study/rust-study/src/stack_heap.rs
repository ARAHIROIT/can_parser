
enum list {
    Node(i32, Box<list>),
    Nil,
}


pub fn run() {
    // let a1: [u8; 700000];
    
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("{:p}", &v1);
    println!("{:p}", &v2);
    println!("Heep  ptr v1: {:?}", v1.as_ptr());
    println!("Heep  LEN v1: {:?}", v1.len());
    println!("Heep capacity v1: {:?}", v1.capacity());
    v1.insert(1, 10);
    println!("Heep v1: {:?}", v1);
    v1.remove(0);
    println!("Heep v1: {:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("Heep  ptr v1: {:?}", v1.as_ptr());
    println!("Heep  LEN v1: {:?}", v1.len());
    println!("Heep capacity v1: {:?}", v1.capacity());
    println!("{:?}", v3);
    println!("Heep  ptr v3: {:?}", v3.as_ptr());
    println!("Heep  LEN v3: {:?}", v3.len());
    println!("Heep capacity v3: {:?}", v3.capacity());
    let t1: (i64, String) = (10, String::from("hello"));
    println!("addres {:p}", &t1);
    println!("Heep  ptr v1: {:?}", t1.1.as_ptr());
    println!("Heep  LEN v1: {:?}", t1.1.len());
    println!("Heep capacity v1: {:?}", t1.1.capacity());
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
    println!("addres {:p}", &b1);
    println!("addres {:p}", b1);


}