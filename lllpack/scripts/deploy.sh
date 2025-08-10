#!/bin/bash
# LogLine lllpack Deployment Script
# Realiza o deployment do container institucional
# ---------------------------------------

set -e

# Constantes
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
LLLPACK_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="${LLLPACK_ROOT}/dist"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOGLINE_CLI="${LLLPACK_ROOT}/runtime/logline_cli"

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
    "STEP")
      echo -e "${BLUE}[${timestamp}] [STEP]${NC} $message"
      ;;
    *)
      echo -e "[${timestamp}] [$level] $message"
      ;;
  esac
}

# Parsing de argumentos
ENVIRONMENT="dev"
PACKAGE_PATH=""
DRY_RUN=false

print_usage() {
  echo -e "${BOLD}LogLine lllpack Deployment Script${NC}"
  echo "Uso: $0 [opções] <arquivo_lllpack>"
  echo "Opções:"
  echo "  -e, --env ENV        Ambiente de destino (dev, staging, prod) (padrão: dev)"
  echo "  -d, --dry-run        Simula o deployment sem executá-lo"
  echo "  -h, --help           Mostra esta mensagem de ajuda"
  echo "Exemplo:"
  echo "  $0 -e prod ../dist/lllpack_20250804_123456.zst"
}

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -e|--env)
      ENVIRONMENT="$2"
      shift 2
      ;;
    -d|--dry-run)
      DRY_RUN=true
      shift
      ;;
    -h|--help)
      print_usage
      exit 0
      ;;
    *)
      if [ -z "$PACKAGE_PATH" ]; then
        PACKAGE_PATH="$1"
      else
        log "ERROR" "Argumento desconhecido ou múltiplos pacotes especificados: $1"
        print_usage
        exit 1
      fi
      shift
      ;;
  esac
done

# Verificar se o pacote foi especificado
if [ -z "$PACKAGE_PATH" ]; then
  log "ERROR" "Nenhum pacote lllpack especificado"
  print_usage
  exit 1
fi

# Verificar se o pacote existe
if [ ! -f "$PACKAGE_PATH" ]; then
  log "ERROR" "Arquivo de pacote não encontrado: $PACKAGE_PATH"
  exit 1
fi

# Cabeçalho
echo -e "${BOLD}LogLine lllpack Deployment${NC}"
echo -e "${BOLD}========================${NC}"
echo

# Verificar dependências
log "STEP" "Verificando dependências..."
for cmd in zstd curl jq; do
  if ! command -v $cmd &> /dev/null; then
    log "ERROR" "Comando '$cmd' não encontrado. Por favor instale-o e tente novamente."
    exit 1
  fi
done

# Verificar presença do LogLine CLI
if [ ! -f "$LOGLINE_CLI" ] && [ ! -f "$(which logline)" ]; then
  log "ERROR" "LogLine CLI não encontrado em $LOGLINE_CLI"
  log "STEP" "Tentando usar versão global..."
  
  if command -v logline &> /dev/null; then
    log "INFO" "Usando LogLine CLI global"
    LOGLINE_CLI="logline"
  else
    log "ERROR" "LogLine CLI não encontrado no PATH. Abortando."
    exit 1
  fi
fi

# Verificar o pacote
log "STEP" "Verificando integridade do pacote..."
PACKAGE_SIZE=$(du -h "$PACKAGE_PATH" | cut -f1)
PACKAGE_HASH=$(sha256sum "$PACKAGE_PATH" | cut -d' ' -f1)

log "INFO" "Pacote: $(basename "$PACKAGE_PATH")"
log "INFO" "Tamanho: $PACKAGE_SIZE"
log "INFO" "Hash SHA-256: $PACKAGE_HASH"

# Extrair metadados do pacote
log "STEP" "Extraindo metadados do pacote..."
TEMP_DIR="/tmp/lllpack_deploy_${TIMESTAMP}"
mkdir -p "$TEMP_DIR"

zstd -d < "$PACKAGE_PATH" | tar -xf - -C "$TEMP_DIR" lllpack.yaml

if [ ! -f "${TEMP_DIR}/lllpack.yaml" ]; then
  log "ERROR" "Manifesto lllpack.yaml não encontrado no pacote"
  rm -rf "$TEMP_DIR"
  exit 1
fi

# Ler metadados do manifesto
PACKAGE_NAME=$(grep -o 'name: "[^"]*"' "${TEMP_DIR}/lllpack.yaml" | cut -d'"' -f2)
PACKAGE_VERSION=$(grep -o 'version: "[^"]*"' "${TEMP_DIR}/lllpack.yaml" | cut -d'"' -f2)
PACKAGE_AUTHOR=$(grep -o 'author: "[^"]*"' "${TEMP_DIR}/lllpack.yaml" | cut -d'"' -f2)

log "INFO" "Nome do pacote: $PACKAGE_NAME"
log "INFO" "Versão: $PACKAGE_VERSION"
log "INFO" "Autor: $PACKAGE_AUTHOR"

# Verificar assinatura se presente
if [ -f "${TEMP_DIR}/FILES.sha256.sig" ] && [ -f "${TEMP_DIR}/keys/public.pem" ]; then
  log "STEP" "Verificando assinatura do pacote..."
  
  if openssl dgst -sha256 -verify "${TEMP_DIR}/keys/public.pem" -signature "${TEMP_DIR}/FILES.sha256.sig" "${TEMP_DIR}/FILES.sha256"; then
    log "INFO" "Assinatura do pacote verificada com sucesso"
  else
    log "ERROR" "Falha na verificação da assinatura do pacote"
    rm -rf "$TEMP_DIR"
    exit 1
  fi
else
  log "WARN" "Pacote não está assinado ou faltam chaves de verificação"
fi

# Limpar diretório temporário
rm -rf "$TEMP_DIR"

# Preparar parametros de deployment
DEPLOY_PARAMS="--package \"$PACKAGE_PATH\" --env $ENVIRONMENT"

if [ "$DRY_RUN" = true ]; then
  DEPLOY_PARAMS="$DEPLOY_PARAMS --dry-run"
  log "INFO" "Executando em modo simulação (dry-run)"
fi

# Selecionar targets de deployment com base no ambiente
case "$ENVIRONMENT" in
  dev)
    TARGETS="local"
    log "INFO" "Ambiente de destino: Desenvolvimento (local)"
    ;;
  staging)
    TARGETS="staging-server"
    log "INFO" "Ambiente de destino: Staging"
    ;;
  prod)
    TARGETS="prod-primary prod-secondary"
    log "INFO" "Ambiente de destino: Produção (primário e secundário)"
    ;;
  *)
    log "ERROR" "Ambiente desconhecido: $ENVIRONMENT"
    exit 1
    ;;
esac

# Iniciar deployment
log "STEP" "Iniciando processo de deployment..."

# Criar span de deployment
log "INFO" "Criando span de deployment..."
SPAN_ID=$("$LOGLINE_CLI" span create \
  --type "lllpack_deployment" \
  --fields "package:$(basename "$PACKAGE_PATH"),environment:$ENVIRONMENT,version:$PACKAGE_VERSION,status:initiating" \
  --output json | jq -r '.span_id')

log "INFO" "Span de deployment criado: $SPAN_ID"

# Executar deployment em cada target
for target in $TARGETS; do
  log "STEP" "Implantando em $target..."
  
  if [ "$DRY_RUN" = true ]; then
    log "INFO" "[SIMULAÇÃO] Executaria: $LOGLINE_CLI deploy $DEPLOY_PARAMS --target $target"
    DEPLOY_RESULT="success"
  else
    # Executar o deployment real
    if "$LOGLINE_CLI" deploy $DEPLOY_PARAMS --target $target; then
      DEPLOY_RESULT="success"
    else
      DEPLOY_RESULT="failed"
      log "ERROR" "Falha no deployment para $target"
    fi
  fi
  
  # Registrar resultado na span
  "$LOGLINE_CLI" span update \
    --id "$SPAN_ID" \
    --fields "target_$target:$DEPLOY_RESULT"
  
  if [ "$DEPLOY_RESULT" = "failed" ]; then
    log "STEP" "Iniciando rollback devido a falha..."
    
    if [ "$DRY_RUN" = true ]; then
      log "INFO" "[SIMULAÇÃO] Executaria rollback em $target"
    else
      "$LOGLINE_CLI" deploy --rollback --target $target
      log "INFO" "Rollback executado em $target"
    fi
    
    # Atualizar span com status de rollback
    "$LOGLINE_CLI" span update \
      --id "$SPAN_ID" \
      --fields "status:rolled_back,rollback_reason:deployment_failure"
    
    log "ERROR" "Deployment abortado devido a falhas"
    exit 1
  fi
done

# Registrar sucesso na span
"$LOGLINE_CLI" span update \
  --id "$SPAN_ID" \
  --fields "status:deployed,deployment_time:$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ")"

# Status final
log "STEP" "Deployment concluído com sucesso!"
echo
echo -e "${GREEN}Pacote $PACKAGE_NAME v$PACKAGE_VERSION implantado em $ENVIRONMENT${NC}"
echo -e "ID do span de deployment: ${BOLD}$SPAN_ID${NC}"
echo

log "INFO" "Para verificar o status do deployment:"
echo -e "  ${BLUE}$LOGLINE_CLI span get --id $SPAN_ID${NC}"
