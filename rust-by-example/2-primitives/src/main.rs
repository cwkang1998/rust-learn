use std::fmt::Display;
use std::mem;


fn reverse(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

fn transpose(in_matrix: Matrix) -> Matrix {
    Matrix(in_matrix.0, in_matrix.2, in_matrix.1, in_matrix.3)
}

// Struct that consists of tuples
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn analyze_slice(slice: &[f64]) {
    println!("first element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    let integer = 3; // default to u32
    let float = 1.2; // default to f64
    let flag = true;
    let some_random_string: &str = "Hello world";
    let some_chars = 'c';

    let mut mutable_state: u64 = 122021;
    mutable_state = 2222222222;

    println!("{integer}");
    println!("{float:.3}");
    println!("{flag}");
    println!("{some_random_string}");
    println!("{some_chars}");
    println!("{mutable_state:#x}",);

    // Literals
    println!("{} {} {} {}", 1_000_000u64, 0b1011001, 0o773, 0x123123);

    // Tuples
    let tuple_no_1 = (342, "dogs");
    println!("{:#?}", tuple_no_1);

    let a_pair = (3, false);
    println!("{a_pair:#?}");
    println!("{:#?}", reverse(a_pair));

    let (a, b) = a_pair;
    println!("{a}, {b}");

    // Matrix
    println!("{:#?}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2));
    println!("transposed:\n{}", transpose(Matrix(1.1, 1.2, 2.1, 2.2)));

    // Arrays and slices
    let some_arr: [f64; 3] = [1f64, 2f64, 3.3];
    let arr_of_ones: [i32; 1000] = [1; 1000];

    println!("{}", some_arr[0]);
    println!("{:#?}", some_arr);
    println!("{}", arr_of_ones[100]);
    println!("memory: {}", mem::size_of_val(&arr_of_ones));

    analyze_slice(&some_arr[1..]);

    println!("safe get arr ele: {}", match arr_of_ones.get(1000) {
        Some(val) => val,
        None => &-1
    })

}
