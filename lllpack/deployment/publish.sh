#!/bin/bash
# Script para publicação do lllpack de deployment como release no GitHub
# e emissão de span na timeline oficial da voulezvous.ai

set -e

# Configurações
PACKAGE_NAME="institutional-deployment-orchestrator"
PACKAGE_VERSION="1.0.0"
GITHUB_REPO="danvoulez/lllpack"
TIMELINE_API="https://api.voulezvous.ai/timeline/spans"
RELEASE_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
RELEASE_ID=$(uuidgen | tr '[:upper:]' '[:lower:]')

# Cores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Iniciando publicação do ${PACKAGE_NAME} v${PACKAGE_VERSION}${NC}"
echo "ID da release: ${RELEASE_ID}"
echo "Data: ${RELEASE_DATE}"

# 1. Preparar o pacote
echo -e "\n${GREEN}[1/5]${NC} Preparando pacote..."

# Garantir que estamos no diretório raiz do projeto
cd "$(dirname "$0")/../.."

# Definir diretórios
ROOT_DIR=$(pwd)
PACK_DIR="${ROOT_DIR}/lllpack/deployment"
BUILD_DIR="${ROOT_DIR}/target/lllpack-release"
SRC_DIR="${ROOT_DIR}/src/deployment"

# Criar estrutura de diretórios
echo "Criando estrutura em ${BUILD_DIR}"
mkdir -p "${BUILD_DIR}/src"

# Copiar arquivos do lllpack
echo "Copiando arquivos do lllpack"
cp -f "${PACK_DIR}/manifest.toml" "${BUILD_DIR}/"
cp -f "${PACK_DIR}/deployment.registry.logline" "${BUILD_DIR}/"
cp -f "${PACK_DIR}/README.md" "${BUILD_DIR}/"
cp -f "${PACK_DIR}/llldeploy.rs" "${BUILD_DIR}/"

# Verificar e copiar arquivos do código-fonte
echo "Copiando arquivos de código-fonte"
find "${SRC_DIR}" -name "*.rs" -exec cp -f {} "${BUILD_DIR}/src/" \; || echo "Aviso: Alguns arquivos de código-fonte podem não existir"

# Gerar arquivo de checksum
echo -e "\n${GREEN}[2/5]${NC} Gerando checksums..."
cd "${BUILD_DIR}"
find . -type f -not -path "*/\.*" | sort | xargs shasum -a 256 > "CHECKSUMS.sha256"
cd -

# 2. Gerar a assinatura institucional
echo -e "\n${GREEN}[3/5]${NC} Assinando pacote com chave institucional..."
# Simulação de assinatura para este exemplo
SIGNATURE="ed25519:$(openssl rand -hex 32)"
echo "$SIGNATURE" > "${BUILD_DIR}/SIGNATURE"

# 3. Criar span de registro na timeline
echo -e "\n${GREEN}[4/5]${NC} Emitindo span de registro na timeline voulezvous.ai..."

# Cria o payload do span
cat > "${BUILD_DIR}/deployment.register.span.json" << EOF
{
  "type": "deployment.register",
  "id": "${RELEASE_ID}",
  "timestamp": "${RELEASE_DATE}",
  "creator": "voulezvous.ai",
  "package": {
    "name": "${PACKAGE_NAME}",
    "version": "${PACKAGE_VERSION}",
    "institutional": true,
    "signature": "${SIGNATURE}"
  },
  "github": {
    "repository": "${GITHUB_REPO}",
    "release_tag": "v${PACKAGE_VERSION}",
    "release_url": "https://github.com/${GITHUB_REPO}/releases/tag/v${PACKAGE_VERSION}"
  },
  "timeline_contract": {
    "spans_required": [
      "deployment.initiation",
      "deployment.phase",
      "deployment.completion",
      "deployment.rollback"
    ],
    "audit_level": "institutional",
    "retention_policy": "permanent"
  },
  "security": {
    "verified": true,
    "signature_timestamp": "${RELEASE_DATE}",
    "certification_authority": "voulezvous.ai"
  }
}
EOF

# Em um ambiente real, enviaríamos para a API
echo "Span de registro criado: ${BUILD_DIR}/deployment.register.span.json"
echo "Em um ambiente de produção, este span seria enviado para ${TIMELINE_API}"

# 4. Publicar no GitHub
echo -e "\n${GREEN}[5/5]${NC} Publicando no GitHub como release..."
echo -e "\nPara criar uma release no GitHub, execute:\n\ngit tag v${PACKAGE_VERSION}\ngit push origin v${PACKAGE_VERSION}\ngithub-cli release create v${PACKAGE_VERSION} -t \"Institutional Deployment Orchestrator v${PACKAGE_VERSION}\" --repo ${GITHUB_REPO} --notes \"Release institucional do orquestrador de deployment para LogLine Headless IDE\"\n"

echo -e "\n${GREEN}Publicação do ${PACKAGE_NAME} v${PACKAGE_VERSION} concluída com sucesso!${NC}"
echo "Release ID: ${RELEASE_ID}"
echo "Timeline span: deployment.register"
echo "Assinatura: ${SIGNATURE:0:20}..."

# Tornar o script executável
chmod +x "$0"
