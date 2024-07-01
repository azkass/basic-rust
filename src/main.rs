fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
    println!("ini adalah unit test")
}

#[test]
fn variabel() {
    let a = 10;
    println!("{}", a);
}

#[test]
fn variabel_mutable() {
    let mut a = 10;
    println!("{}", a);
    a = 20;
    println!("{}", a);
}

#[test]
fn variabel_shadowing() {
    let a = 10;
    println!("{}", a);
    let a = 20;
    println!("{}", a);
}

#[test]
fn tipe_data() {
    let a = 10; // type inference
    let b:f64 = 10.0; // explicit type
    let c:i32 = -10; // signed integer
    let d:u32 = 10; // unsigned integer
    let e = "hello";
    let f = true;
    println!("{} {} {} {} {} {}", a, b, c, d, e, f);
}

#[test]
fn integer_conersion(){
    let a:i32 = 1000000;
    let b:i64 = a as i64;
    println!("{}", b);

    let c:i8 = a as i8;
    println!("{}", c);
}

#[test]
fn numeric_operation(){
    let a = 10;
    let b = 3;
    let c = a + b;
    let d = a - b;
    let e = a * b;
    let f = a / b;
    let g = a % b;
    println!("{} {} {} {} {}", c, d, e, f, g);
}

#[test]
fn augmented_assignments(){
    let mut a = 10;
    a += 3;
    println!("{}", a);

    a -= 3;
    println!("{}", a);

    a *= 3;
    println!("{}", a);

    a /= 3;
    println!("{}", a);

    a %= 3;
    println!("{}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}

#[test]
fn comparison(){
    let result = 10 > 20;
    println!("{}", result);
}

#[test]
fn boolean_operator(){
    let a = true;
    let b = false;
    let c = a && b;
    let d = a || b;
    let e = !a;
    println!("{} {} {}", c, d, e);
}

#[test]
fn character(){
    let a = 'a';
    let b = 'b';
    println!("{} {}", a, b);
}

#[test]
fn tuple(){
    let data : (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
}

#[test]
fn destructing_tuple(){
    let data : (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, _) = data;
    println!("{} {}", a, b);
}

#[test]
fn mutable_tuple(){
    let mut data : (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
    
    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

#[test]
fn unit(){
    let a = ();
    println!("{:?}", a);
}

#[test]
fn array(){
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let first = array[0];
    let second = array[1];
    println!("{} {}", first, second);
}

#[test]
fn mutable_array(){
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array(){
    let array: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4]
    ];
    println!("{:?}", array);

    let first = array[0][0];
    let second = array[0][1];
    let third = array[1][0];
    let fourth = array[1][1];
    println!("{} {} {} {}", first, second, third, fourth);
}

#[test]
fn contant(){
    const PI: f64 = 3.14;
    println!("{}", PI);
}