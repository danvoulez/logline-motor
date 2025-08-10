# Arquitetura do lllpack

## Visão Geral

O lllpack é um container institucional completo baseado em LogLine, projetado para encapsular não apenas código, mas constituição, contratos, agentes, timeline e memória vetorial. Diferentemente de containers tradicionais como Docker, o lllpack implementa um regime institucional auditável, onde cada operação é registrada como um span na timeline, permitindo rastreabilidade completa e reversibilidade das operações.

## Princípios Fundamentais

1. **Soberania Institucional**: Todo lllpack é uma entidade institucional soberana com identidade, contratos e governança própria.

2. **Rastreabilidade Total**: Cada operação dentro do lllpack gera spans na timeline, criando um registro imutável e auditável.

3. **Reversibilidade Garantida**: A timeline permite reverter qualquer operação, proporcionando um mecanismo robusto de rollback.

4. **Agentes como Cidadãos**: Os agentes LLM são cidadãos institucionais de pleno direito, com capacidades declaradas por contrato.

5. **Verificabilidade**: Todo pacote é assinado e verificável, com hash completo e causalidade rastreável.

## Componentes Principais

### 1. Manifesto (lllpack.yaml)

O arquivo `lllpack.yaml` define a identidade e estrutura do pacote institucional, incluindo:

- Metadados (nome, versão, descrição)
- Autores e identidade institucional
- Configuração de runtime (VM, plugins, recursos)
- Entrypoint e sequência de boot
- Contratos fundamentais
- Agentes integrados
- Triggers automáticos
- Configuração de verificação e assinatura

### 2. Constituição e Contratos

Os arquivos `.lll` na pasta `contracts/` definem:

- Princípios fundamentais de governança
- Direitos e garantias das entidades institucionais
- Procedimentos para emendas e evolução
- Regras para gerenciamento de spans e timeline

### 3. Sistema de Spans e Timeline

O coração do lllpack é seu sistema de spans e timeline:

- **Spans**: Unidades fundamentais de execução, cada uma representando uma operação institucional com metadados completos, atores, campos de dados e hashes de causalidade.
- **Timeline**: Registro cronológico e causal de todos os spans executados, permitindo auditoria completa e reversão de operações.
- **Colapsos**: Mecanismo para otimizar a timeline, preservando causalidade mas reduzindo o tamanho do histórico.

### 4. Runtime e VM

O LogLine VM (`runtime/logline_vm`) é responsável por:

- Executar spans conforme definido nos contratos
- Manter a integridade da timeline
- Aplicar limites de recursos
- Garantir isolamento e segurança

### 5. Plugins WASM

Os plugins WebAssembly na pasta `runtime/plugins/` estendem as capacidades do sistema:

- `validator.wasm`: Valida contratos e spans
- `span_merger.wasm`: Otimiza a timeline via colapso de spans
- `trigger_cron.wasm`: Executa spans em horários programados

### 6. Agentes LLM

Os agentes na pasta `agents/` são entidades LLM com:

- Identidade e personalidade definidas por contrato
- Permissões específicas para acesso a timeline, spans e memória
- Capacidade de despachar spans como cidadãos institucionais
- SLA institucional garantido pelo contrato

### 7. Embeddings e Memória Vetorial

O diretório `embeddings/` contém:

- Índices vetoriais para recuperação semântica
- Memória institucional persistente
- Vetores de conhecimento específicos do domínio

### 8. Triggers

Os triggers em `triggers/` definem:

- Execução automática de spans em eventos específicos
- Monitoramento contínuo de condições
- Tarefas agendadas e manutenção

## Fluxo de Execução

1. **Inicialização**: O script `run.sh` inicia o LogLine VM com a sequência de boot.
2. **Boot Sequence**: `boot_sequence.lll` é executado, inicializando a timeline, carregando contratos e ativando agentes.
3. **Operação Normal**: Spans são criados, validados e executados, registrando todas as operações na timeline.
4. **Execução de Agentes**: Os agentes LLM operam como cidadãos institucionais, consultando a timeline e despachando spans.
5. **Triggers**: Eventos programados ou baseados em condições disparam execuções automáticas.

## Diferenças em Relação a Containers Tradicionais

| Aspecto | Docker | lllpack |
|---------|--------|---------|
| Unidade básica | Processos e sistema de arquivos | Spans e timeline |
| Estado | Implícito no filesystem | Explícito na timeline |
| Reversibilidade | Limitada (rebuild ou restore) | Total via timeline |
| Auditabilidade | Logs externos | Nativa via spans |
| Identidade | Container ID | Identidade institucional |
| Agentes LLM | Não nativos | Cidadãos institucionais |
| Governança | Via orquestrador externo | Constitucional interna |

## Integração com LogLine Headless IDE

O lllpack é projetado para integração perfeita com o LogLine Headless IDE:

1. O **DeployOrchestrator** no módulo `deploy.rs` gerencia o ciclo de vida completo do lllpack.
2. O **WasmSandbox** no módulo `sandbox.rs` executa plugins do lllpack em ambiente seguro.
3. O **Timeline Manager** coordena a timeline do lllpack com a timeline global do sistema.
4. Os **Agentes Institucionais** são integrados ao sistema de agentes do IDE.

## Considerações de Segurança

- Cada lllpack é executado em seu próprio sandbox WASM.
- As permissões são explicitamente declaradas no manifesto.
- A assinatura digital verifica a integridade do pacote.
- A timeline preserva a causalidade, impedindo modificações fraudulentas.
- Os limites de recursos protegem contra consumo excessivo.

## Conclusão

O lllpack representa uma evolução significativa em relação a containers tradicionais, oferecendo não apenas isolamento e portabilidade, mas uma verdadeira governança institucional com rastreabilidade, reversibilidade e auditabilidade completas. Como um "container filosófico", ele encapsula não apenas código, mas a própria essência da instituição digital soberana.
