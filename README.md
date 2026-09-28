# Garnet

Garnet é uma linguagem experimental acompanhada de um compilador escrito em Rust. O projeto tem finalidade didática: estudar como uma linguagem e seus compiladores são construídos, começando pela análise do código-fonte e avançando por etapas.

## Estado do projeto

O repositório está em desenvolvimento. No momento, o programa implementa um lexer, um parser inicial e uma representação de árvore sintática abstrata (AST). Ainda não executa os programas Garnet nem gera código de máquina ou código para outra plataforma.

### Etapas de um compilador

- [x] **Análise léxica (lexer):** percorre o texto e reconhece palavras-chave (`int`, `print`, `if`, `else`), identificadores, números inteiros, strings, operadores e pontuação. Caracteres desconhecidos causam panic.
- [x] **Análise sintática (parser):** reconhece declarações `int::nome = expressão;`, atribuições `nome = expressão;` e chamadas `print(expressão);`.
- [x] **Construção da AST:** representa declarações, atribuições, impressão e expressões literais ou identificadores.
- [ ] **Análise semântica:** validar declarações e usos de variáveis, tipos e outras regras da linguagem.
- [ ] **Representação intermediária (IR):** transformar a AST em uma forma intermediária apropriada às próximas etapas.
- [ ] **Otimização:** simplificar ou melhorar o programa antes da geração de código.
- [ ] **Geração de código:** produzir código de máquina, bytecode ou código para outra linguagem/plataforma.
- [ ] **Execução:** executar o código gerado ou interpretar diretamente os programas Garnet.

Os itens marcados indicam funcionalidades presentes no código, mas não necessariamente uma implementação completa ou robusta. Por exemplo, o lexer reconhece `+`, `<` e `>`, e a AST prevê expressões binárias, mas o parser atualmente só consome um literal ou identificador por expressão. Da mesma forma, `if` e `else` são reconhecidos como tokens, mas ainda não são aceitos como comandos pelo parser.

## Estrutura

```text
rust-compiler/
├── Cargo.toml
└── src/
    ├── ast/       # Tipos da AST
    ├── lexing/    # Tokens e lexer
    ├── parsing/   # Parser
    └── main.rs    # Demonstração do pipeline atual
main.gnt           # Exemplo de código na sintaxe pretendida
```

## Executar

É necessário ter o [Rust](https://www.rust-lang.org/tools/install) instalado. Na pasta do repositório, execute:

```sh
cargo run --manifest-path rust-compiler/Cargo.toml
```

O programa processa atualmente uma amostra definida diretamente em `rust-compiler/src/main.rs` e exibe os tokens e a AST no terminal. Ele ainda não lê `main.gnt` como entrada.

## Sintaxe reconhecida pelo parser atual

```garnet
int::idade = 20;
idade = 21;
print(idade);
print("Olá, Garnet!");
```

Declarações com `int::` aceitam atualmente uma expressão simples: um número inteiro, uma string ou um identificador. A linguagem e sua gramática ainda estão sujeitas a mudanças.
