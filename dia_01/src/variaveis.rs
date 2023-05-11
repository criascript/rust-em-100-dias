pub fn int() {
    println!("Variáveis são imutáveis por padrão");
    // int -> i8, i16, i32, i64, i128
    // uint -> u8, u16, u32, u64, u128
    // declarando uma variável int com valor
    let _int: i32 = 1;
    println!("int: {}", _int);

    println!("Para tornar uma variável mutável, é necessário usar a palavra-chave mut");
    let mut _int: i32 = 2;

    // retornando o valor da variável int
    println!("int: {}", _int);
}

pub fn float() {
    // float -> f32, f64
    println!("float -> f32, f64");
    // declarando uma variável float com valor
    let _float: f32 = 1.6;
    println!("float: {}", _float);
}

pub fn char() {
    // char -> 'a', 'b', 'c', 'd'
    println!("char -> 'a', 'b', 'c', 'd'");
    // declarando uma variável char vazia
    let _char: char = '\0';
    println!("char: {}", _char);
    // declarando uma variável char com valor
    let _char: char = 'a';
    println!("char: {}", _char);
}

pub fn bool() {
    // bool -> true, false
    println!("bool -> true, false");
    // declarando uma variável bool
    let _bool: bool = true;
    println!("bool: {}", _bool);
}

pub fn string() {
    // string -> "Hello, World!"
    println!("string -> \"Hello, World!\"");
    // declarando uma variável string com valor
    let string: String = "Hello, World!".to_owned();
    println!("string: {}", string);
}

pub fn array() {
    // array -> [1, 2, 3, 4, 5]
    println!("array -> [1, 2, 3, 4, 5]");
    // declarando uma variável array vazia
    let _array: Vec<i32> = vec![];
    println!("array: {:?}", _array);
    // declarando uma variável array com valores (tamanho fixo)
    let _array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", _array);
}

pub fn tuple() {
    // tuple -> (1, 2, 3, 4, 5)
    println!("tuple -> (1, 2, 3, 4, 5)");
    // declarando uma variável tuple com valores
    let _tuple: (i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5);
    println!("tuple: {:?}", _tuple);
}

pub fn slice() {
    // slice -> &[1, 2, 3, 4, 5]
    println!("slice -> &[1, 2, 3, 4, 5]");
    // declarando uma variável slice vazia
    let _slice: &[i32] = &[];
    println!("slice: {:?}", _slice);
    // declarando uma variável slice com valores
    let _slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("slice: {:?}", _slice);
}