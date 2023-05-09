


fn main() {
    // Variáveis são imutáveis por padrão

    // int -> i8, i16, i32, i64, i128
    // uint -> u8, u16, u32, u64, u128
    let a: i32 = 1;

    // float -> f32, f64
    let b: f32 = 1.6;

    // Para tornar uma variável mutável, é necessário usar a palavra-chave mut
    let mut b: i32 = 2;

    // char -> 'a', 'b', 'c', 'd'
    let c: char = 'a';

    // bool -> true, false
    let d: bool = true;

    // string -> "Hello, World!"
    let e: String = "Hello, World!".to_owned();

    // array -> [1, 2, 3, 4, 5]

    // declarando array vazio com macro
    let v = vec![];

    // declarando array com valores
    let f: [i32; 5] = [1, 2, 3, 4, 5];

    // tuple -> (1, 2, 3, 4, 5)

    // declarando uma tupla vazia
    let t = ();

    // declarando uma tupla com valores
    let g: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);

    // slice -> &[1, 2, 3, 4, 5]
    let h: &[i32] = &[1, 2, 3, 4, 5];

    // struct -> struct User { name: String, age: i32 }
    struct User {
        name: String,
        age: i32,
    }

    // enum -> enum Option<T> { Some(T), None }
    enum Option<T> {
        Some(T),
        None,
    }

    // function -> fn sum(a: i32, b: i32) -> i32 { a + b }
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // pattern -> let (x, y) = (1, 2);
    let (x, y) = (1, 2);

}
