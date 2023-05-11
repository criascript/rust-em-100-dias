# ** Dia 02 - Rust em 100 dias**

## O que aprendi hoje?

### 1. Aprendi a usar o comando `cargo new` para criar um projeto rust.

### 2. Aprendi a usar o comando `cargo run` para executar o projeto rust.

### 3. Aprendi a fazer workspaces com o cargo e especifiquei como funciona no README principal.

### 4. Aprendi conceitos de Macros no rust:
    As macros no Rust podem ser definidas como derive, attribute e function-like.

    - Derive macros: São macros que podem ser usadas em structs e enums. Exemplo: `#[derive(Debug)]`

    - Attribute macros: São macros que podem ser usadas em qualquer item. Exemplo: `#[test]`

    - Function-like macros: São macros que podem ser usadas como funções. Exemplo: `println!()`

### 5. Aprendi a usar o comando `cargo doc --open` para gerar a documentação do projeto e abrir no navegador.
    Para documentar basta usar o comentário `///!` antes da função, struct, enum, etc. Exemplo:
    ```rust
    ///! Função que soma dois números
    ///!
    ///! # Exemplo
    ///!
    ///! ```
    ///! let x = 1;
    ///! let y = 2;
    ///! let z = soma(x, y);
    ///! assert_eq!(z, 3);
    ///! ```
    fn soma(x: i32, y: i32) -> i32 {
        x + y
    }
    ```
    Para documentar o projeto, basta usar o comentário `//!` no início do arquivo `lib.rs` ou `main.rs`.

### 6. Aprendi sobre assincronismo com o tokio-rs. 
   O Rust por padrão não tem suporte a assincronismo, mas existem bibliotecas que implementam esse recurso. A mais famosa é o tokio-rs. Para usar o tokio-rs, basta adicionar a dependência.

    Para usar o tokio-rs, basta adicionar o `#[tokio::main]` antes da função `main()`. Exemplo:
    ```rust
    #[tokio::main]
    async fn main() {
        println!("Hello, world!");
    }
    ```

### 7. Aprendi sobre o Axum 
    O Axum é um framework para desenvolvimento de API em Rust. Para usar o Axum, basta adicionar a dependência.

    Para usar o Axum, basta adicionar o `#[tokio::main]` antes da função `main()`.

### 8. Conheci a parking_lot 
    A parking_lot é uma biblioteca que implementa locks de forma mais eficiente que a std. Para usar a parking_lot, basta adicionar a dependência no arquivo `Cargo.toml`

### 9. Vi um pouco do conceito de serde
    O serde é uma biblioteca que implementa serialização e desserialização de forma mais eficiente que a std. Para usar o serde, basta adicionar a dependência no arquivo `Cargo.toml`

### 10. Aprendi a utilizar o serde_json
    O serde_json é uma biblioteca que implementa serialização e desserialização de json de forma mais eficiente que a std. Para usar o serde_json, basta adicionar a dependência no arquivo `Cargo.toml`