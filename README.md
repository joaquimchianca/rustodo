# Rustodo
Uma ferramenta de gerenciamento de tarefas no Terminal.

## Vídeos
### Demonstração do programa


### Explicação do código



## Descrição
Esse programa foi criado como projeto final da disciplina de Programação em Rust, ministrada pelo professor Wedson
no semestre 2024.1 na Universidade Federal do Rio Grande do Norte (UFRN). A ideia é sintetizar todo conhecimento aprendido em sala de aula e do livro oficial do Rust nesse aplicativo.

O programa consiste em um aplicativo na interface na linha de comando em que funciona como uma lista de tarefas. 
Nesse programa é possível: 
 - Adicionar tarefas
 - Listar tarefas (do dia, do mês, da semana e todas tarefas finalizadas)
 - Marcar tarefas como finalizadas
 - Remover tarefas
 - Reagendar tarefas

## Ferramentas utilizadas
 - `SQLite`: armazenamento das tarefas
 - `Cargo`: gerenciamento de dependências
 - Bibliotecas Rust (crates):
    - `nanoid`: criação de ids aleatórios
    - `colored`: utilização de cores no terminal
    - `rusqlite`: comunicação Rust <-> SQLite
    - `chrono`: manipulação de datas
    - `clap`: ferramentas para criação de CLI apps.
 

## Como executar o projeto
Existem dois ambientes para utilizar o Rustodo: produção e desenvolvimento.

### Produção
 1. Baixar o arquivo .zip `rustodo.zip` e descompactá-lo
 2. Definir uma variável de ambiente com o caminho correto para esse arquivo.
A definição de variáveis de ambiente pode alterar para cada sistema operacional, aqui vai uma referência para te ajudar:
 - [Windows](https://learn.microsoft.com/en-us/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14))
 - [Linux](https://www.digitalocean.com/community/tutorials/how-to-read-and-set-environmental-and-shell-variables-on-linux-pt)
 - [MacOS](https://support.apple.com/pt-br/guide/terminal/apd382cc5fa-4f58-4449-b20a-41c53c006f8f/mac)

Feito isso, teste o comando `rustodo --help`. Se definido corretamente a seguinte mensagem aparecerá:

```
rustodo 1.0
Joaquim Chianca | github.com/joaquimchianca
Your Task Management CLI

USAGE:
    rustodo [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add     Add a new task
    edit    Change the task date through ID.
    help    Print this message or the help of the given subcommand(s)
    ls      List tasks
    ok      Mark a task as done.
    rm      Remove a task
```

### Desenvolvimento
 1. Clone este repositório
    ```
    git clone git@github.com:joaquimchianca/rustodo.git
    ```
 2. Compile o programa (necessário ter o [cargo instalado](https://doc.rust-lang.org/cargo/getting-started/installation.html))
    ```
    cargo build --release
    ```
 3. Navegue até a página `target/release`
    ```
    cd target/release
    ```
 4. Use o comando `./rustodo -h` e a mesma mensagem deve aparecer para você, caso tenha feito corretamente

## Estrutura de pacotes
```
rustodo/
├── src/
│   ├── main.rs             # Configura o CLI e processa comandos.
│   ├── commands/           # Módulos específicos para cada comando.
│   │   ├── add.rs          # Adicionar tarefas.
│   │   ├── list.rs         # Listar tarefas.
│   │   ├── edit.rs         # Editar data das tarefas
│   │   ├── remove.rs       # Remover tarefas.
│   │   └── done.rs         # Marcar tarefas como concluídas.
│   ├── models/             # Definições de estruturas de dados.
│   │   └── task.rs         # Struct Task e lógica relacionada.
│   ├── db/                 # Banco de dados.
│   │   └── mod.rs          # Funções para interação com o banco de dados SQLite.
│   └── utils/              # Funções utilitárias.
│       └── mod.rs          # Implementações de utilitários.
├── Cargo.toml              # Configurações do cargo e dependências do projeto.
└── README.md               # Descrição do projeto, uso, e instruções de instalação.
```
