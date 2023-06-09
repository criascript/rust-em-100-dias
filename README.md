# Rust em 100 dias

Este repositório tem como objetivo documentar o que se pode aprender em 100 dias estudando uma nova linguagem de programação.
A linguagem escolhida foi Rust, pelo seu potencial de crescimento nos próximos anos e pela minha curiosidade. 

> É válido salientar que não sou iniciante em programação, logo não terei muitas das dificuldades que iniciantes enfrentam ao aprender novas linguagens. Caso você esteja acompanhando e não sinta que está evoluindo igualmente, apenas relaxe e siga no seu ritmo :) 

## Workspace

Para facilitar o desenvolvimento, criei um workspace com um projeto para cada dia. Para criar um workspace, basta executar o comando:

```bash
cargo init
```
e depois adicionar os projetos no arquivo `Cargo.toml`:

```toml

[[workspaces]]
members = [
    "dia_01",
    "dia_02",
]
```

### Criar novos dias 

```bash
cargo new "dia_xx"
```