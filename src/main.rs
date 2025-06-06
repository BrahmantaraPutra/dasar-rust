fn main() {
    println!("Hello, world!");
    println!("angga gimang");
    print!("1");
    print!("2");
}

//biar bisa ngedit tanpa ngeganggu function main pake #[test] dulu di depan
#[test]
fn angga(){
    println!("angga");
}

//variable imutable
#[test]
fn variable() {
    let name= "Brahmantara";
    println!("angga dan {}", name );
}

//mutable variable di tambahin mut di depan let
#[test]
fn mutable(){
    let mut name = "anggga";
    println!("aku dan {}", name);

    name = "hitam";
    println!("kamu itu {}", name);
}

//cara shadowing variable
#[test]
fn shadowing() {
    let name= "Brahmantara";
    println!("angga dan {}", name );

    let name = 10;
    println!("1 + 7 = {}", name);
}
/*                                  // -> buat comment sebaris   /* */ -> buat comment dari atas sampe bawah
#[test]
fn shadowing() {
    //let name= "Brahmantara";
    println!("angga dan {}", name );

    let name = 10;
    println!("1 + 7 = {}", name);
}
    */


//data type 
#[test]
fn explocit() {
    let age : i64 = 17;
    println!("ini explocit {}", age);
}

#[test]
fn number() {
    let age : i64 = 17;
    println!("{}", age);

    let b : f64 = 10.5;
    println!("{}", b);
}

//convert data
#[test]
fn numberconvert() {
    let a : i8 = 17;
    println!("{}", a);

    let b : i16 = a as i16 ;
    println!("{}", b);

    let c : i32 = b as i32 ;
    println!("{}", c);
}

//numeric operator
#[test]
fn numeric() {
    let a = 10;
    let b = 2;
    let c = a + b ;
    println!("{}", c);
    let d = a - b;
    println!("{}", d);
    let e = a * b;
    println!("{}", e);
    let f = a / b;
    println!("{}", f);
    let g = a % b;
    print!("{}", g);
}

//augment Assigments
#[test]
fn aug() {
let mut a = 10;
println!("a = {}", a);

a += 10;
println!("{}", a);

a -= 10;
println!("{}", a);

a *= 10;
println!("{}", a);

a /= 10;
println!("{}", a);

a %= 10;
println!("{}", a);
}

#[test]
fn boolean (){
    let a = true;
    let b : bool = false;
    print!("{} {}", a, b);
}