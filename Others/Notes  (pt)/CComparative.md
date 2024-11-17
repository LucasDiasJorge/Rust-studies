### **1. Declaração de Variáveis**

- **C**: As variáveis são mutáveis por padrão.
- **Rust**: As variáveis são imutáveis por padrão. Para permitir mutação, usamos `mut`.

#### Exemplo:
**C**:
```c
#include <stdio.h>

int main() {
    int x = 10;
    x = 20; // Pode alterar o valor
    printf("%d\n", x);
    return 0;
}
```

**Rust**:
```rust
fn main() {
    let mut x = 10; // "mut" permite mutação
    x = 20;
    println!("{}", x);
}
```

---

### **2. Funções**

- **C**: Usa o tipo de retorno explicitamente.
- **Rust**: Similar ao C, mas funções retornam a última expressão sem `return` (opcional).

#### Exemplo:
**C**:
```c
#include <stdio.h>

int soma(int a, int b) {
    return a + b;
}

int main() {
    printf("%d\n", soma(5, 7));
    return 0;
}
```

**Rust**:
```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b // Última expressão é retornada
}

fn main() {
    println!("{}", soma(5, 7));
}
```

---

### **3. Ponteiros e Referências**

- **C**: Gerencia diretamente a memória usando ponteiros.
- **Rust**: Usa referências, garantindo segurança de memória sem _dangling pointers_.

#### Exemplo:
**C**:
```c
#include <stdio.h>

void altera_valor(int *x) {
    *x = 42; // Acessa e modifica o valor via ponteiro
}

int main() {
    int y = 10;
    altera_valor(&y); // Passa o endereço
    printf("%d\n", y);
    return 0;
}
```

**Rust**:
```rust
fn altera_valor(x: &mut i32) {
    *x = 42; // Acessa e modifica o valor via referência mutável
}

fn main() {
    let mut y = 10;
    altera_valor(&mut y); // Passa referência mutável
    println!("{}", y);
}
```

---

### **4. Gerenciamento de Memória**

- **C**: Deve-se alocar e liberar manualmente a memória com `malloc` e `free`.
- **Rust**: Gerencia memória automaticamente através de um _ownership system_.

#### Exemplo:
**C**:
```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    int *ptr = (int *)malloc(sizeof(int));
    *ptr = 5; // Aloca e usa a memória
    printf("%d\n", *ptr);
    free(ptr); // Libera a memória manualmente
    return 0;
}
```

**Rust**:
```rust
fn main() {
    let x = Box::new(5); // Aloca memória automaticamente
    println!("{}", *x); // Rust libera a memória quando `x` sai do escopo
}
```

---

### **5. Controle de Concorrência**

- **C**: Usa threads da biblioteca padrão (`pthread`).
- **Rust**: Usa o módulo `std::thread` para _threads_ seguras e eficientes.

#### Exemplo:
**C**:
```c
#include <pthread.h>
#include <stdio.h>

void *imprime(void *arg) {
    printf("Thread!\n");
    return NULL;
}

int main() {
    pthread_t t;
    pthread_create(&t, NULL, imprime, NULL);
    pthread_join(t, NULL); // Aguarda a thread finalizar
    return 0;
}
```

**Rust**:
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Thread!");
    });
    handle.join().unwrap(); // Aguarda a thread finalizar
}
```

---

### **6. Estruturas de Dados**

- **C**: Usa `struct` para criar estruturas.
- **Rust**: Também usa `struct`, mas com funcionalidades adicionais como _traits_.

#### Exemplo:
**C**:
```c
#include <stdio.h>

struct Ponto {
    int x;
    int y;
};

int main() {
    struct Ponto p = {10, 20};
    printf("x: %d, y: %d\n", p.x, p.y);
    return 0;
}
```

**Rust**:
```rust
struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Ponto { x: 10, y: 20 };
    println!("x: {}, y: {}", p.x, p.y);
}
```

---

### **7. Enums**

- **C**: Enums são apenas inteiros com nomes simbólicos.
- **Rust**: Enums podem ter dados associados e são mais poderosos.

#### Exemplo:
**C**:
```c
#include <stdio.h>

enum Estado { Ligado, Desligado };

int main() {
    enum Estado estado = Ligado;
    if (estado == Ligado) {
        printf("Ligado\n");
    }
    return 0;
}
```

**Rust**:
```rust
enum Estado {
    Ligado,
    Desligado,
}

fn main() {
    let estado = Estado::Ligado;
    match estado {
        Estado::Ligado => println!("Ligado"),
        Estado::Desligado => println!("Desligado"),
    }
}
```

---

### **8. Manipulação de Erros**

- **C**: Não há tratamento nativo de erros, geralmente se usa códigos de retorno.
- **Rust**: Usa tipos como `Result` e `Option` para tratar erros de forma segura.

#### Exemplo:
**C**:
```c
#include <stdio.h>

int divide(int a, int b, int *resultado) {
    if (b == 0) return -1; // Retorna erro
    *resultado = a / b;
    return 0; // Sucesso
}

int main() {
    int resultado;
    if (divide(10, 0, &resultado) == -1) {
        printf("Erro: divisão por zero\n");
    }
    return 0;
}
```

**Rust**:
```rust
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("divisão por zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(resultado) => println!("{}", resultado),
        Err(erro) => println!("Erro: {}", erro),
    }
}
```
