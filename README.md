# Auto-Presser

Um aplicativo simples de auto-clique desenvolvido com [Slint](https://slint.dev/), oferecendo uma interface gráfica intuitiva para automatizar pressionamentos de teclas.

## Funcionalidades

- Interface gráfica minimalista e fácil de usar
- Configuração do intervalo entre pressionamentos (em milissegundos)
- Seleção da tecla a ser pressionada
- Iniciar/Parar o auto-presser com um único clique
- Indicador visual de status (ativo/inativo)

## Requisitos

- Rust Stable ou Nightly
- Slint (última versão)
- Sistema operacional: Windows, Linux ou macOS

## Instalação

1. Clone o repositório:
```bash
git clone https://github.com/ViniciusRed/auto-presser.git
cd auto-presser
```

2. Compile o projeto:
```bash
cargo build --release
```

3. Execute o aplicativo:
```bash
cargo run --release
```

## Como Usar

1. Abra o aplicativo
2. Configure o intervalo desejado entre os pressionamentos
3. Selecione a tecla que deseja pressionar automaticamente
4. Clique em "Iniciar" para começar o auto-presser
5. Para interromper, clique em "Parar"

## Estrutura do Projeto

```
auto-presser/
├── src/
│   ├── main.rs         # Ponto de entrada do aplicativo e interface
├── Cargo.toml
└── README.md
```

## Tecnologias Utilizadas

- **Rust**: Linguagem de programação principal
- **Slint**: Framework para criação da interface gráfica
- **enigo**: Biblioteca para simulação de entrada de teclado

## Contribuindo

Contribuições são bem-vindas! Por favor, sinta-se à vontade para enviar pull requests ou abrir issues para sugerir melhorias ou reportar bugs.

## Licença

Este projeto está licenciado sob a MIT License - veja o arquivo LICENSE para detalhes.

## Aviso Legal

Este software é fornecido "como está", sem garantias. O uso inadequado de auto-clickers ou auto-presser pode violar os termos de serviço de alguns aplicativos ou jogos. Use com responsabilidade.
