#!/bin/bash
# LogLine lllpack Run Script
# Executa o container institucional completo
# ---------------------------------------

set -e

# Constantes
LLLPACK_ROOT="$(dirname "$(dirname "$(readlink -f "$0")")")"
LOGLINE_VM="${LLLPACK_ROOT}/runtime/logline_vm"
BOOT_SEQUENCE="${LLLPACK_ROOT}/spans/boot_sequence.lll"
TIMELINE="${LLLPACK_ROOT}/timeline/main.timeline"
LOG_FILE="${LLLPACK_ROOT}/logs/lllpack_run.log"

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Função para logging
log() {
  local level="$1"
  local message="$2"
  local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ")
  
  case "$level" in
    "INFO")
      echo -e "${GREEN}[${timestamp}] [INFO]${NC} $message"
      ;;
    "WARN")
      echo -e "${YELLOW}[${timestamp}] [WARN]${NC} $message"
      ;;
    "ERROR")
      echo -e "${RED}[${timestamp}] [ERROR]${NC} $message"
      ;;
    "SYSTEM")
      echo -e "${BLUE}[${timestamp}] [SYSTEM]${NC} $message"
      ;;
    *)
      echo -e "[${timestamp}] [$level] $message"
      ;;
  esac
  
  # Append to log file
  echo "[$timestamp] [$level] $message" >> "$LOG_FILE"
}

# Criar diretório de logs se não existir
mkdir -p "$(dirname "$LOG_FILE")"

# Cabeçalho
echo -e "${BOLD}LogLine lllpack Runtime Executor${NC}"
echo -e "${BOLD}=================================${NC}"
echo

# Verificar presença da VM
log "SYSTEM" "Verificando presença do LogLine VM..."
if [ ! -f "$LOGLINE_VM" ] && [ ! -f "$(which logline_vm)" ]; then
  log "ERROR" "LogLine VM não encontrada em $LOGLINE_VM"
  log "SYSTEM" "Tentando usar versão global..."
  
  if command -v logline_vm &> /dev/null; then
    log "INFO" "Usando LogLine VM global"
    LOGLINE_VM="logline_vm"
  else
    log "ERROR" "LogLine VM não encontrada no PATH. Abortando."
    exit 1
  fi
fi

# Verificar manifesto lllpack
log "SYSTEM" "Verificando manifesto lllpack..."
if [ ! -f "${LLLPACK_ROOT}/lllpack.yaml" ]; then
  log "ERROR" "Manifesto lllpack não encontrado em ${LLLPACK_ROOT}/lllpack.yaml"
  exit 1
fi

# Verificar boot sequence
log "SYSTEM" "Verificando sequência de boot..."
if [ ! -f "$BOOT_SEQUENCE" ]; then
  log "ERROR" "Sequência de boot não encontrada em $BOOT_SEQUENCE"
  exit 1
fi

# Verificar timeline
log "SYSTEM" "Verificando timeline..."
if [ ! -f "$TIMELINE" ]; then
  log "WARN" "Timeline principal não encontrada em $TIMELINE"
  log "SYSTEM" "Criando nova timeline..."
  touch "$TIMELINE"
fi

# Função para exibir status dos spans
display_spans_status() {
  local spans_count=$(grep -c "\"type\": \"span\"" "$TIMELINE")
  local latest_span=$(grep "\"type\": \"span\"" "$TIMELINE" | tail -1)
  local latest_span_type=$(echo "$latest_span" | grep -o "\"span_type\": \"[^\"]*\"" | cut -d'"' -f4)
  local latest_span_timestamp=$(echo "$latest_span" | grep -o "\"timestamp\": \"[^\"]*\"" | cut -d'"' -f4)
  
  log "INFO" "Total de spans: $spans_count"
  log "INFO" "Último span: $latest_span_type em $latest_span_timestamp"
}

# Executar boot sequence
log "SYSTEM" "Iniciando execução da sequência de boot..."
log "INFO" "Boot sequence: $BOOT_SEQUENCE"
log "INFO" "Timeline: $TIMELINE"

# Executar LogLine VM com boot sequence
"$LOGLINE_VM" run --boot "$BOOT_SEQUENCE" --timeline "$TIMELINE" --tenant voulezvous --output-format json || {
  log "ERROR" "Falha na execução da VM. Código de saída: $?"
  exit 1
}

log "SYSTEM" "Sequência de boot concluída"

# Ativar triggers
log "SYSTEM" "Ativando triggers..."
if [ -d "${LLLPACK_ROOT}/triggers" ]; then
  for trigger in "${LLLPACK_ROOT}/triggers"/*.lll; do
    if [ -f "$trigger" ]; then
      trigger_name=$(basename "$trigger" .lll)
      log "INFO" "Ativando trigger: $trigger_name"
      "$LOGLINE_VM" trigger --file "$trigger" --timeline "$TIMELINE" --tenant voulezvous &
    fi
  done
fi

# Exibir status da timeline
log "SYSTEM" "Status da timeline:"
display_spans_status

# Mostrar agentes ativos
log "SYSTEM" "Agentes ativos:"
if [ -d "${LLLPACK_ROOT}/agents" ]; then
  for agent in "${LLLPACK_ROOT}/agents"/*.lll; do
    if [ -f "$agent" ]; then
      agent_name=$(basename "$agent" .lll)
      log "INFO" "Agente: $agent_name"
    fi
  done
fi

# Status final
log "SYSTEM" "lllpack está em execução"
log "INFO" "Use CTRL+C para encerrar o processo"
log "INFO" "Os spans continuarão a ser registrados na timeline: $TIMELINE"

# Manter processo ativo
tail -f "$LOG_FILE"
