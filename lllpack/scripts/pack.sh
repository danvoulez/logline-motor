#!/bin/bash
# LogLine lllpack Packaging Script
# Empacota o container institucional em formato portátil
# ---------------------------------------

set -e

# Constantes
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
LLLPACK_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="${LLLPACK_ROOT}/dist"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
DEFAULT_OUTPUT="${OUTPUT_DIR}/lllpack_${TIMESTAMP}.zst"
TEMP_DIR="/tmp/lllpack_build_${TIMESTAMP}"

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
  
  case "$level" in
    "INFO")
      echo -e "${GREEN}[INFO]${NC} $message"
      ;;
    "WARN")
      echo -e "${YELLOW}[WARN]${NC} $message"
      ;;
    "ERROR")
      echo -e "${RED}[ERROR]${NC} $message"
      ;;
    "STEP")
      echo -e "${BLUE}[STEP]${NC} $message"
      ;;
    *)
      echo -e "[$level] $message"
      ;;
  esac
}

# Parsing de argumentos
OUTPUT_FILE="${DEFAULT_OUTPUT}"
SIGN=true

print_usage() {
  echo -e "${BOLD}LogLine lllpack Packaging Script${NC}"
  echo "Uso: $0 [opções]"
  echo "Opções:"
  echo "  -o, --output FILE    Define o arquivo de saída (padrão: ${DEFAULT_OUTPUT})"
  echo "  -n, --no-sign        Não assina o pacote"
  echo "  -h, --help           Mostra esta mensagem de ajuda"
}

while [[ "$#" -gt 0 ]]; do
  case $1 in
    -o|--output)
      OUTPUT_FILE="$2"
      shift 2
      ;;
    -n|--no-sign)
      SIGN=false
      shift
      ;;
    -h|--help)
      print_usage
      exit 0
      ;;
    *)
      log "ERROR" "Argumento desconhecido: $1"
      print_usage
      exit 1
      ;;
  esac
done

# Criar diretórios necessários
mkdir -p "${OUTPUT_DIR}"
mkdir -p "${TEMP_DIR}"

# Cabeçalho
echo -e "${BOLD}LogLine lllpack Packaging${NC}"
echo -e "${BOLD}======================${NC}"
echo

# Verificar dependências
log "STEP" "Verificando dependências..."
for cmd in rsync zstd sha256sum; do
  if ! command -v $cmd &> /dev/null; then
    log "ERROR" "Comando '$cmd' não encontrado. Por favor instale-o e tente novamente."
    exit 1
  fi
done

# Verificar manifesto
log "STEP" "Verificando manifesto lllpack..."
if [ ! -f "${LLLPACK_ROOT}/lllpack.yaml" ]; then
  log "ERROR" "Manifesto lllpack.yaml não encontrado em ${LLLPACK_ROOT}"
  exit 1
fi

# Ler versão do manifesto
VERSION=$(grep -o 'version: "[^"]*"' "${LLLPACK_ROOT}/lllpack.yaml" | cut -d'"' -f2)
NAME=$(grep -o 'name: "[^"]*"' "${LLLPACK_ROOT}/lllpack.yaml" | cut -d'"' -f2)

if [ -z "$VERSION" ] || [ -z "$NAME" ]; then
  log "ERROR" "Não foi possível extrair nome ou versão do manifesto lllpack.yaml"
  exit 1
fi

log "INFO" "Empacotando $NAME versão $VERSION"

# Copiar arquivos para diretório temporário
log "STEP" "Copiando arquivos para diretório temporário..."
rsync -a --exclude 'dist' --exclude '.git' "${LLLPACK_ROOT}/" "${TEMP_DIR}/"

# Gerar hash do manifesto
log "STEP" "Gerando hashes de verificação..."
MANIFEST_HASH=$(sha256sum "${TEMP_DIR}/lllpack.yaml" | cut -d' ' -f1)
log "INFO" "Hash do manifesto: ${MANIFEST_HASH}"

# Gerar lista de arquivos e hashes
log "STEP" "Gerando inventário de arquivos..."
find "${TEMP_DIR}" -type f | sort | while read file; do
  rel_path="${file#$TEMP_DIR/}"
  hash=$(sha256sum "$file" | cut -d' ' -f1)
  echo "${hash} ${rel_path}" >> "${TEMP_DIR}/FILES.sha256"
done

log "INFO" "Inventário gerado com $(wc -l < "${TEMP_DIR}/FILES.sha256") arquivos"

# Assinar o pacote se solicitado
if [ "$SIGN" = true ]; then
  log "STEP" "Assinando o pacote..."
  
  # Verificar se temos chaves privadas
  if [ -f "${LLLPACK_ROOT}/keys/private.pem" ]; then
    log "INFO" "Usando chave privada existente"
    cp "${LLLPACK_ROOT}/keys/private.pem" "${TEMP_DIR}/keys/"
  else
    log "WARN" "Chave privada não encontrada, gerando nova chave..."
    mkdir -p "${TEMP_DIR}/keys"
    openssl genrsa -out "${TEMP_DIR}/keys/private.pem" 2048
    openssl rsa -in "${TEMP_DIR}/keys/private.pem" -pubout -out "${TEMP_DIR}/keys/public.pem"
  fi
  
  # Criar assinatura do inventário
  openssl dgst -sha256 -sign "${TEMP_DIR}/keys/private.pem" -out "${TEMP_DIR}/FILES.sha256.sig" "${TEMP_DIR}/FILES.sha256"
  log "INFO" "Pacote assinado com sucesso"
else
  log "WARN" "Assinatura de pacote desativada"
fi

# Compactar o pacote
log "STEP" "Compactando o pacote..."
pushd "${TEMP_DIR}" > /dev/null
tar -cf - . | zstd -T0 -19 > "${OUTPUT_FILE}"
popd > /dev/null

# Gerar hash do pacote final
FINAL_HASH=$(sha256sum "${OUTPUT_FILE}" | cut -d' ' -f1)
FINAL_SIZE=$(du -h "${OUTPUT_FILE}" | cut -f1)

# Limpar diretório temporário
log "STEP" "Limpando arquivos temporários..."
rm -rf "${TEMP_DIR}"

# Mostrar informações finais
log "INFO" "Pacote criado com sucesso:"
echo -e "${BOLD}Nome:${NC} $NAME"
echo -e "${BOLD}Versão:${NC} $VERSION"
echo -e "${BOLD}Arquivo:${NC} ${OUTPUT_FILE}"
echo -e "${BOLD}Tamanho:${NC} ${FINAL_SIZE}"
echo -e "${BOLD}Hash SHA-256:${NC} ${FINAL_HASH}"
echo -e "${BOLD}Assinado:${NC} $SIGN"
echo

log "STEP" "Pacote lllpack criado com sucesso!"
echo -e "Para executar: ${GREEN}logline run ${OUTPUT_FILE}${NC}"
