# The Dan Voulez LogLine Motor

## Visão geral

**The Dan Voulez LogLine Motor** é um conjunto de ferramentas e serviços
modulares voltados para automação, governança e execução de spans no
ecossistema LogLine.  O projeto foi organizado como um workspace
multi‑crate escrito em Rust e inclui componentes para API HTTP, serviço
de streaming via Server‑Sent Events (SSE), CLI, mecanismo de simulação,
interpretador de comandos e um pacote institucional (\*lllpack\*) que
permite executar tudo de forma reprodutível em uma máquina virtual do
LogLine.  Cada crate isolada implementa uma parte específica da
funcionalidade, mas todas compartilham o mesmo domínio: permitir que
empresas e agentes interajam com a timeline de spans, assinem contratos
e executem regras automatizadas.

O projeto foi concebido para facilitar a manutenção, observabilidade e
extensibilidade.  Os desenvolvedores podem adicionar novas regras,
agentes e integrações sem alterar o núcleo, enquanto as equipes de
operacionalização podem implantar o sistema em contêineres ou em VMs
dedicadas sem dependências externas de SaaS.

## Principais características

* **Workspace multi‑crate** – O diretório `The\ Dan\ Voulez\ LogLine\ Motor` é
  um *workspace* com diversos pacotes Rust.  Há crates para a API
  (`logline_api`), serviço de streaming (`streaming`), mecanismo de
  simulação (`simulate`), parser de comandos (`parser`), repositório de
  contratos (`contracts`), indexação vetorial (`vectorindex`), geração
  de documentação (`docs_gen`) e núcleo (`core`).
* **Runtime institucional** – O subdiretório `lllpack/` contém um
  manifesto YAML (`lllpack.yaml`) e scripts (`run.sh` e `pack.sh`) que
  embalam o runtime completo com VM, triggers e contratos.  Ele
  especifica a sequência de boot, contratos fundamentais, agentes
  integrados e triggers automáticos.
* **APIs e Streaming** – `logline_api` expõe uma API HTTP
  JSON/RESTful para interagir com a timeline (criação e consulta de
  spans, contratos e ações).  O crate `streaming` implementa SSE para
  transmitir eventos em tempo real.
* **CLI e simulador** – O binário CLI (`cli`) permite interagir com o
  serviço a partir da linha de comando, invocando simulações,
  orchestrations e regras.  O crate `simulate` oferece um motor para
  rodar e avaliar simulações, armazenando resultados em memória ou
  persistindo em provedores como Supabase.
* **Parser e contratos** – O crate `parser` usa a biblioteca `nom`
  para analisar comandos textuais em comandos estruturados; o crate
  `contracts` define estruturas e regras para contratos e spans.

## Requisitos

Para compilar e rodar o projeto, são necessários:

1. **Rust versão estável (>= 1.89.0)** – Use o `rustup` para instalar
   o compilador.  O time de desenvolvimento recomenda atualizar para a
   versão estável mais recente com `rustup update stable`【164055566245624†L17-L24】.
   O projeto depende de crates (por exemplo, `clap` e `native‑tls`) que
   exigem Rust 1.80 ou superior.
2. **Make e ferramentas de build** – No Linux, instale o pacote
   `build‑essential`; no macOS, instale Xcode Command Line Tools (via
   `xcode‑select --install`).
3. **Supabase (opcional)** – Para persistir resultados de simulação
   além da memória, configure um projeto no Supabase e crie uma tabela
   `simulations` com os campos correspondentes.  Exporte
   `SUPABASE_URL` e `SUPABASE_KEY` no ambiente ou crie um arquivo
   `.env`.

## Instalação

1. **Clone ou extraia o repositório.**
   ```bash
   git clone https://github.com/seu_usuario/logline_motor.git
   cd logline_motor
   # ou extraia o arquivo loglinemotor_patched_2_fixed.zip fornecido
   ```
2. **Instale o Rust**.  Se ainda não tiver, execute:
   ```bash
   curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
   source "$HOME/.cargo/env"
   rustup update stable  # garante versão estável mais recente【164055566245624†L17-L24】
   ```
3. **Configure as variáveis de ambiente**.  Se usar Supabase, defina:
   ```bash
   export SUPABASE_URL="https://seu-projeto.supabase.co"
   export SUPABASE_KEY="sua-api-key"
   ```
   Para logs detalhados, defina `RUST_LOG=info`.

## Compilação

O projeto é um *workspace*, então o comando `cargo` manipula todos os
crates de uma vez:

1. **Verificar erros rapidamente** – Use `cargo check` para validar o
   código sem gerar binário.  Esse comando é rápido porque não
   produz executável, permitindo verificar se o projeto continua
   compilando enquanto você programa【248553439861977†L297-L310】.
2. **Compilar para depuração** – Execute:
   ```bash
   cargo build
   ```
   Por padrão, `cargo build` compila todas as bibliotecas e binários do
   workspace e coloca os artefatos em `target/debug`【542974053541379†L169-L171】.  Você
   pode executar os binários diretamente após a compilação.
3. **Compilar versão otimizada** – Quando for distribuir ou
   benchmarkar, use:
   ```bash
   cargo build --release
   ```
   Este comando ativa otimizações e grava os executáveis em
   `target/release`【248553439861977†L326-L336】.

## Execução dos serviços

Depois de compilar, você pode iniciar os serviços específicos:

* **API HTTP** – Inicia o servidor RESTful na porta definida
  (por padrão 3000):
  ```bash
  cargo run --release -p logline_api
  ```
* **Streaming SSE** – Transmite spans em tempo real via Server‑Sent
  Events na porta 4000:
  ```bash
  cargo run --release -p streaming
  ```
* **CLI** – Permite executar comandos, simular entidades e aplicar
  regras a partir do terminal:
  ```bash
  cargo run --release -p cli -- comando argumento1 argumento2
  ```

### Executar o lllpack (Modo Institucional)

O `lllpack/` encapsula o runtime institucional.  Para executar a VM
com a timeline:

1. Torne os scripts executáveis (se ainda não estiverem):
   ```bash
   cd lllpack/scripts
   chmod +x run.sh pack.sh
   ```
2. **Inicialize a VM e a timeline:**
   ```bash
   ./run.sh
   ```
   O script carrega `lllpack.yaml`, inicializa a VM, carrega os
   contratos (`constituição`, `onboarding`, `regras_tenant`) e agentes
   definidos no manifesto, e mantém a timeline em
   `timeline/main.timeline`.
3. **Empacote o lllpack:**
   ```bash
   ./pack.sh
   ```
   Gera um arquivo `.zst` assinado em `lllpack/dist/` para distribuição.

## Estrutura de diretórios

```
├── core/                # Biblioteca central com tipos e utilidades
├── runtime/             # Ambiente de execução assíncrono
├── streaming/           # Servidor SSE para streaming de spans
├── logline_api/         # API HTTP/JSON
├── simulate/            # Motor de simulação e armazenamento
├── parser/              # Parser baseado em nom
├── contracts/           # Contratos e regras em LLL
├── docs_gen/            # Geração automática de documentação
├── vectorindex/         # Indexação vetorial de embeddings
├── ideas/               # Gerenciamento de ideias e sugestões
├── lllpack/             # Pacote institucional (manifesto, scripts, agentes)
│   ├── scripts/
│   ├── spans/
│   ├── triggers/
│   └── timeline/
└── cli/                 # Cliente de linha de comando
```

## Configuração adicional

* **Supabase** – Crie uma tabela `simulations` com campos
  `id`, `entity_id`, `round`, `timestamp`, `metrics`, `decision` e
  defina as políticas de permissões conforme necessário.  Ajuste as
  variáveis `SUPABASE_URL` e `SUPABASE_KEY` para apontar para seu
  projeto.
* **Registro de logs** – O projeto utiliza o crate `env_logger`.  Para
  ativar logs, defina `RUST_LOG=info` antes de rodar os binários ou
  adicione uma configuração no `.env`.
* **Features opcionais** – Alguns crates possuem flags de feature
  (por exemplo, `mem‑store` no simulate).  Ative‑as via `--features` no
  comando `cargo build` se desejar compilar módulos específicos.

## Testes

Execute todos os testes unitários e de integração com:

```bash
cargo test
```

O Cargo automaticamente compila e executa os testes para todos os
crates do workspace.

## Solução de problemas

* **Falha de compilação devido ao Rust desatualizado** – Se você
  receber mensagens indicando que alguns crates requerem versões
  superiores (por exemplo, `native‑tls` ou `clap`), atualize o Rust
  com `rustup update stable`【164055566245624†L17-L24】.
* **Erros de ambiente** – Verifique se `SUPABASE_URL` e
  `SUPABASE_KEY` estão definidos.  Se não estiver usando Supabase,
  configure o modo em memória ou ajuste o código do crate `simulate`.
* **Parser quebrando** – O parser utiliza a API atual do `nom`.  Caso
  modifique combinadores, importe `nom::Parser` e chame `.parse(input)`.

## Licença

O conteúdo deste projeto é distribuído sob a licença MIT, salvo
indicação em contrário em arquivos específicos.

## Agradecimentos

Este projeto é inspirado e sustentado pela comunidade LogLine e pelo
time de engenharia da VoulezVous.  Agradecemos às contribuidoras e
contribuidores que mantêm a base de código atualizada.