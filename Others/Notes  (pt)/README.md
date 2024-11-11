### 1. Conceitos Básicos de Ownership em Rust

Rust usa um sistema de *ownership* para gerenciar a memória automaticamente sem precisar de um coletor de lixo (*garbage collector*). Cada valor em Rust possui uma variável chamada *dona*, e ela é responsável pela limpeza de sua memória quando sai do escopo. Isso evita vazamentos e oferece segurança na manipulação de dados.

#### Três Regras Fundamentais do Ownership

1. Cada valor em Rust tem um *owner* (dono).
2. Cada valor só pode ter um *owner* por vez.
3. Quando o *owner* sai do escopo, o valor é automaticamente desalocado da memória.

#### Exemplo Básico

```rust
fn main() {
    let s1 = String::from("Hello, Rust!"); // s1 é o dono de "Hello, Rust!"
    let s2 = s1; // Transfere a propriedade de s1 para s2
    // println!("{}", s1); // Isso causaria um erro pois s1 perdeu a propriedade
    println!("{}", s2); // Correto, pois s2 agora é o dono
}
```

Quando tentamos usar `s1` após transferir a propriedade para `s2`, o compilador não permite, pois `s1` já não é mais o dono. Esse mecanismo evita que uma mesma área de memória seja liberada duas vezes.

### 2. Borrowing: Empréstimo de Dados

Nem sempre queremos transferir a propriedade. Muitas vezes, precisamos "emprestar" o dado para uma função ou outra parte do código sem perder a posse. O *borrowing* permite isso.

#### Regras do Borrowing

1. Podemos ter múltiplas referências imutáveis ao mesmo dado.
2. Podemos ter apenas uma referência mutável ao dado por vez.
3. As referências imutáveis e mutáveis não podem coexistir no mesmo escopo.

Essas regras ajudam a evitar *data races* e garantem segurança ao acessar dados concorrentes.

#### Exemplo de Borrowing com Referências Imutáveis

```rust
fn main() {
    let s1 = String::from("Hello, Rust!");

    let len1 = calculate_length(&s1); // Passa uma referência imutável de s1
    let len2 = calculate_length(&s1); // Podemos ter múltiplas referências imutáveis

    println!("Length: {}, {}", len1, len2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Nesse exemplo, `&s1` é uma referência imutável para `s1`, que permite calcular o comprimento sem transferir a posse.

#### Exemplo de Borrowing com Referências Mutáveis

```rust
fn main() {
    let mut s1 = String::from("Hello");

    change(&mut s1); // Passa uma referência mutável para `change`
    println!("{}", s1); // s1 agora é "Hello, Rust!"
}

fn change(s: &mut String) {
    s.push_str(", Rust!");
}
```

Aqui, `s1` é passado como uma referência mutável `&mut s1` para a função `change`, que permite modificar o conteúdo da string original.

### 3. Lifetimes: Gerenciando a Vida Útil das Referências

Rust precisa garantir que todas as referências sejam válidas enquanto usadas. Para isso, introduzimos o conceito de *lifetimes* (tempo de vida), que nada mais é do que uma anotação para garantir que a referência não ultrapasse o tempo de vida do dado referenciado.

#### Exemplo Simples de Lifetimes

Vamos usar um exemplo onde passamos referências para uma função que retorna uma das referências:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
```

Aqui, o `'a` é uma anotação de lifetime. Ele diz ao compilador que `x`, `y` e o retorno têm o mesmo *lifetime*, ou seja, estarão válidos durante o mesmo período. Isso permite ao Rust garantir que não haja referências inválidas ou fora do escopo.

### 4. Exemplos Práticos para Fixação

#### Transferência de Propriedade com Vetores

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1; // v1 perde a propriedade

    println!("{:?}", v2);
    // println!("{:?}", v1); // Erro, pois v1 não é mais o dono
}
```

#### Borrowing para Funções com Variáveis Complexas

```rust
fn main() {
    let mut scores = vec![10, 20, 30];
    add_score(&mut scores, 40);
    println!("{:?}", scores);
}

fn add_score(scores: &mut Vec<i32>, score: i32) {
    scores.push(score);
}
```

#### Misturando Referências Imutáveis e Mutáveis (Erro)

```rust
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; // Erro: já existem referências imutáveis ativas

    println!("{}, {}, {}", r1, r2, r3);
}
```

Nesse exemplo, tentar criar uma referência mutável `r3` enquanto ainda existem referências imutáveis `r1` e `r2` é inválido.

### 5. Quando Usar Qual Modo de Acesso?

- Use **propriedade** se você quer que uma variável tenha o controle exclusivo sobre o valor.
- Use **referências imutáveis** para múltiplos acessos de leitura a uma variável sem transferir propriedade.
- Use **referências mutáveis** quando precisar alterar o valor, mas garantir que apenas um acesso mutável ocorra ao mesmo tempo.

### 6. Recapitulando os Principais Pontos

- **Ownership** permite a Rust gerenciar a memória de forma automática.
- **Borrowing** permite compartilhar acesso a dados sem transferir a propriedade.
- **Lifetimes** garantem que referências sejam válidas enquanto usadas.

Com essa base, você já possui um conhecimento sólido sobre como Rust gerencia memória com *ownership*, *borrowing* e *lifetimes*! Isso proporciona alta performance e segurança na manipulação de dados e torna o desenvolvimento mais robusto, especialmente em sistemas críticos.