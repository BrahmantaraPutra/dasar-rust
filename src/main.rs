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


#[test]
fn comper (){
    let a = 10;
    let b = 20;

    let result : bool  = a > b;
    print!("{}", result);

    /*let result : bool  = a < b;
    print!("{}", result);

    let result : bool  = a >= b;
    print!("{}", result);

    let result : bool  = a <= b;
    print!("{}", result);

    let result : bool  = a == b;
    print!("{}", result);*/
}


//pake (&&, ||, !)
#[test]
fn operatorbl (){
    let absen = 75;
    let nilai = 800;

    let lulus  = absen >= 70;
    let lulus_nilai_akhir = nilai >= 70;

    let lulus_final= lulus && lulus_nilai_akhir;
    print!("{}", lulus_final);
}

#[test]
fn char_type(){
    let char1 : char = 'a';
    let char2 : char = 'b';

    print!("{} {}", char1, char2);
}

//tuple
#[test]
fn tuple(){
    let data : (i32,f64,bool) = (10,7.7,true);
    print!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    print!("{} {} {}", a, b, c);
}

//tuple
#[test]
fn dec_tuple(){
    let data : (i32,f64,bool) = (10,7.7,true);
    print!("{:?}", data);

    let (_, b, c) = data; 
    print!("{} {}",b, c);
}

//mutable tuple
#[test]
fn tuple_lagi(){
    let mut data : (i32,f64,bool) = (10,7.7,true);
    print!("{:?}", data);

    let (a, b, c) = data; 
    print!("{} {} {}",a, b, c);

    data.0 = 20;
    data.1 = 35.9;
    data.2 = false;
    print!("{:?}", data);
}

fn unit(){
    println!("hello");
}

//tuple tapi gaada isinya
#[test]
fn tuple_kosong(){
    let result: () = unit();
    println!("{:?}", result);

    let angga: () = ();
    println!("{:?}", angga);
}

//array
#[test]
fn array(){
    let array: [i32; 3] = [1,2,3];
    
    print!("{:?}", array);
}

//cara akses array
#[test]
fn array_akses(){
    let array: [i32; 7] = [1,2,3,4,5,6,7];
    
    let a = array[0];
    let b = array[1];
    let c = array[2];
    let d = array[3];
    let e = array[4];
    let f = array[5];
    let g = array[6];

    print!("{:?} {:?}", a, g);
}

//mutable array + ngitung jarak arraynya
#[test]
fn array_mut(){
    let mut array: [i32; 7] = [1,2,3,4,5,6,7];
    
    let a = array[0];
    let g = array[6];
    print!("{} {}", a, g);

    array[0] = 10;
    array[6] = 20;
    print!("{:?}", array);

    let jarak: usize = array.len();
    print!("{}", jarak);
}

//array 2 dimensi
#[test]
fn two_d_array(){
    let two : [[i32;2];2]= [
        [1,2],
        [3,4]
    ];
    println!("{:?}", two);
    println!("{}", two[0][0]);
    println!("{}", two[0][1]);
    println!("{}", two[1][0]);
    println!("{}", two[1][1]);
}

//array 3 dimensi (dimensi selanjutnya tinggal di sesuain aja)
#[test]
fn arrayangga(){
    let matrix : [[i32;3];3]= [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];
    println!("{:?}", matrix);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][2]);
    println!("{}", matrix[2][0]);
}


//constant 
const MAXIMUM:i32 = 100;

#[test]
fn cons(){
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}


//cara stak dan heap menejemen memory di rust
#[test]
fn stak_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("purta");

    println!("{}, {}", a, b);
}

fn function_b(){    
    let a = 20;
    let b = String::from("agus waluyo");

    println!("{}, {}", a, b);
}
