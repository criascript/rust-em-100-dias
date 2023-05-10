mod estruturas_rep;
mod variaveis;

fn main() {
    // chamando variaveis::int();
    variaveis::int();

    // chamando variaveis::float();
    variaveis::float();

    // chamando variaveis::char();
    variaveis::char();

    // chamando variaveis::bool();
    variaveis::bool();

    // chamando variaveis::string();
    variaveis::string();

    // chamando variaveis::array();
    variaveis::array();

    // chamando variaveis::tuple();
    variaveis::tuple();

    // chamando variaveis::slice();
    variaveis::slice();

    // chamando estruturas_rep::while_tradicional();
    estruturas_rep::while_tradicional();

    // chamando estruturas_rep::for_tradicional();
    estruturas_rep::for_tradicional();

    // chamando estruturas_rep::loop_ate_10();
    estruturas_rep::loop_ate_10();


    // struct -> struct User { name: String, age: i32 }
    struct User {
        name: String,
        age: i32,
    }

    fn say_hello_1(user: User) {
        println!("Hello, {}!", user.name);
        print!("You are {} years old.", user.age);
    }

    // Desestruturando uma struct
    fn say_hello_2(User { name, age }: User) {
        println!("Hello, {}!", name);
        print!("You are {} years old.", age);
    }

    struct User2(String, i32);

    fn say_hello_3(user: User2) {
        println!("Hello, {}!", user.0);
        print!("You are {} years old.", user.1);
    }

    // Desestruturando uma struct
    fn say_hello_4(User2 ( name, age ): User2) {
        println!("Hello, {}!", name);
        print!("You are {} years old.", age);
    }

    // Metodo para a struct User
    impl User {
        fn say_hello(&self) {
            println!("Hello, {}!", self.name);
            print!("You are {} years old.", self.age);
        }

        fn change_name(&mut self, name: String) {
            self.name = name;
        }

        fn kys(self) {
            println!("{} is dead.", self.name);
        }
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

    fn tuple_1((a, b): (i32, i32)) -> i32 {
        a + b
    }

    fn tuple_2(t : (i32, i32)) -> i32 {
        t.0 + t.1
    }

    // pattern -> let (x, y) = (1, 2);
    let (x, y) = (1, 2);

}
