pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let i1 = 1;
    let i2 =  i1;
    println!("{} {}", i1, i2);
    println!("addres i1 {:p}", &i1);
    println!("addres i2 {:p}", &i2);


    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("addres i1 {:p}", &sl1);
    println!("addres i2 {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("addres s3 {:p}", &s3);
    println!("addres s4 {:p}", &s4);
    println!("Heep mmeroy s3{:?}", s3.as_ptr());
    println!("Heep mmeroy s4{:?}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    //take_ownershp(s5);
    let sss = take_ownershp(s5);
    println!("{}", sss);

    let s6 = String::from("hhello");
    println!("Heep mmeroy s6{:?}", s6.as_ptr());
    let s7 = take_giveback_ownership(s6);
    println!("{}", s7);
    println!("Heep mmeroy s7{:?}", s7.as_ptr());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("s8  {} {}",s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}",s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}",s10 ,r1, r2);

    let  mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);
    




}







fn take_ownershp(s: String) -> String{
    println!("{}", s);
    let s11 = s + "hello";
    s11
}

fn take_giveback_ownership(s: String) -> String{
    s

}

fn calculate_length(s: &String)-> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_wrold")
}