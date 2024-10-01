fn main() {
    println!("Hello, world!");
}

#[test]
fn unit_test() {
    println!("ini adalah unit test")
}

#[test]
fn variabel() {
    let a:u8 = 10;
    println!("{}", a);
}

#[test]
fn variabel_mutable() {
    let mut a:u8 = 10;
    println!("{}", a);
    a = 20;
    println!("{}", a);
}

#[test]
fn variabel_shadowing() {
    let a:u8 = 10;
    println!("{}", a);
    let a:u8 = 20;
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
    let a:u8 = 10;
    let b:u8 = 3;
    let c:u8 = a + b;
    let d:u8 = a - b;
    let e:u8 = a * b;
    let f:u8 = a / b;
    let g:u8 = a % b;
    println!("{} {} {} {} {}", c, d, e, f, g);
}

#[test]
fn augmented_assignments(){
    let mut a:u8 = 10;
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
    let array: [u8; 5] = [1, 2, 3, 4, 5];
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
    let array: [[u8; 2]; 2] = [
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
fn constant(){
    const PI: f64 = 3.14;
    println!("{}", PI);
}

#[test]
fn variabel_scope(){
    let a:u8 = 10; // variabel scope
    {
        println!("{}", a);

        let b = 20;
        println!("{}", b);
    }
    // println!("{}", b); -> error
}

#[test]
fn string_literal(){
    let a: &str = "  hello  "; // fix string size with space
    let trim: &str = a.trim(); // meskipun &str yang disimpan dalam variable a sudah trim, &str akan tetap menyimpan "  hello  ", sedangkan yang disimpan di trim adalah "hello" dengan &str baru
    println!("{}", a);
    println!("{}", trim);

    // let mut b: &str = "azkas"; b = "azkas"
    // b = "azkas2"; b = "azkas2" 
    // println!("{}", b); -> hasil print = azkas2
    // meskipun data variable b berubah dari "azkas" menjadi "azkas2", &str tetap menyimpan "azkas", karena yang terjadi adalah mengganti value, bukan mengubah value
}

#[test]
fn string(){
    let mut a: String = String::from("hello"); // Heap: hello. Harus mutable karena akan diubah
    a.push_str(" world"); // Heap: hello world
    println!("{}", a);

    let b: String = a.replace("world", "dunia"); // Heap baru: hello dunia
    println!("{}", b);
    println!("{}", a); // a masih menyimpan hello world
}

#[test]
fn ownership_rules(){
    // a tidak dapat diakses disini karena belum dideklarasikan
    let a: String = String::from("hello");
    {
        // b tidak dapat diakses disini karena belum dideklarasikan
        let b: String = String::from("world");
        println!("{}", b);
        println!("{} {}", a, b);
    } // b sudah out of scope maka b dihapus dan tidak dapat diakses lagi
    println!("{}", a);
    // println!("{}", b); -> error
} // a sudah out of scope maka a dihapus dan tidak dapat diakses lagi

#[test]
fn data_copy(){
    let a:u8 = 10; // Stack
    let mut b:u8 = a; // copy data dari a ke b
    println!("{} {}", a, b);

    b = 20; // b diubah
    println!("{} {}", a, b); // a tidak berubah
}

#[test]
fn ownership_movement(){
    let a: String = String::from("hello"); // Heap
    print!("{}", a);

    let b: String = a; // ownership dari a dipindahkan ke b
    // println!("{}", a); -> error karena a sudah tidak memiliki ownership dan sudah dihapus
    println!("{}", b);
}

#[test]
fn clone(){
    let a: String = String::from("hello");
    let b: String = a.clone();
    println!("{} {}", a, b);
}

#[test]
fn if_else(){
    let a:u8 = 10;
    if a > 10 {
        println!("lebih besar dari 10");
    } else if a == 10 {
        println!("sama dengan 10");
    } else if a < 10 {
        println!("lebih kecil dari 10");
    } else {
        println!("tidak diketahui");
    }
}

#[test]
fn let_if(){
    let a:u8 = 10;
    let b:&str = if a > 10 {
        "lebih besar dari 10"
    } else if a == 10 {
        "sama dengan 10"
    } else if a < 10 {
        "lebih kecil dari 10"
    } else {
        "tidak diketahui"
    };
    println!("{}", b);

    // Menggunakan std::mem::size_of untuk mengetahui ukuran memori dari variabel
    println!("Ukuran memori dari a (u8): {} bytes", std::mem::size_of::<u8>());
    println!("Ukuran memori dari b (&str): {} bytes", std::mem::size_of::<&str>());
    println!("Ukuran memori nilai dari b: {} bytes", std::mem::size_of_val(b));

    println!("{}", b);
}

#[test]
fn loop_expression(){
    let mut a:u8 = 0;
    loop {
        a += 1;
        if a > 10 {
            break;
        } else if a % 2 == 0 {
            continue;
        }
        println!("a = {}", a);
    }
}

#[test]
fn loop_label(){
    let mut a: u8 = 1;
    'outer: loop{
        let mut i: u8 = 1;
        loop{
            if a > 5{
                break 'outer;
            }
            println!("{} + {} = {}", a, i, a + i);
            i += 1;
            if i > 5 {
                break;
            }
        }
        a += 1;
    }
}

#[test]
fn while_loop(){
    let mut a: u8 = 0;
    while a < 10 {
        println!("a = {}", a);
        a += 1;
    }
}