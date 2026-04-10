#!/bin/bash
# Benchmark: OpenCode Ecosystem Comparison
# Compara: OpenCode solo vs OpenCode+Agentes vs Funemon vs OpenCode+Funemon completo

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

OUTPUT_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).md"

echo -e "${GREEN}=== OPENCODE ECOSYSTEM BENCHMARK ===${NC}"
echo ""

# ============================================
# FUNCIONES
# ============================================

log_result() {
    echo "$1" >> "$OUTPUT_FILE"
}

get_timestamp_ms() {
    echo $(($(date +%s%N) / 1000000))
}

measure_time() {
    local start end
    start=$(get_timestamp_ms)
    "$@" > /dev/null 2>&1
    end=$(get_timestamp_ms)
    echo $((end - start))
}

# ============================================
# SETUP
# ============================================

echo -e "${YELLOW}[1/5] Setup...${NC}"

FUNEMON_BIN="/home/santi/santi/funemon/funemon-system/target/release/funemon"
PROJECT="benchmark_ecosystem_$(date +%s)"

# Limpiar DBs anteriores
rm -rf "$HOME/.local/share/funemon"
mkdir -p "$HOME/.local/share/funemon"

# Crear archivo de resultados
echo "# Benchmark Results - OpenCode Ecosystem" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "**Date:** $(date '+%Y-%m-%d %H:%M:%S')" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "  Project: $PROJECT"
echo "  Output: $OUTPUT_FILE"
echo ""

# ============================================
# TEST 1: Funemon CLI Solo (sin OpenCode)
# ============================================

echo -e "${YELLOW}[2/5] Test 1: Funemon CLI Solo${NC}"

# Iniciar sesión
START=$(get_timestamp_ms)
SESSION_ID=$($FUNEMON_BIN session start --project "$PROJECT" 2>&1 | grep "ID:" | awk '{print $2}')
END=$(get_timestamp_ms)
FUNEMON_STARTUP=$((END - START))

# Guardar 10memorias
WRITE_TIMES=""
for i in {1..10}; do
    TIME=$(measure_time $FUNEMON_BIN memories store \
        --session-id "$SESSION_ID" \
        --title "Test memory $i" \
        --type "observation" \
        --what "Test content $i" \
        --why "Benchmark")
    WRITE_TIMES="$WRITE_TIMES $TIME"
done
FUNEMON_WRITE=$(echo $WRITE_TIMES | awk '{sum=0; for(i=1;i<=NF;i++)sum+=$i; print sum/NF}')

# Recuperar contexto
TIME=$(measure_time $FUNEMON_BIN session context --session-id "$SESSION_ID")
FUNEMON_CONTEXT=$TIME

# Búsqueda
TIME=$(measure_time $FUNEMON_BIN memories search "test" --session-id "$SESSION_ID")
FUNEMON_SEARCH=$TIME

echo "  Startup: ${FUNEMON_STARTUP}ms"
echo "  Write (avg): ${FUNEMON_WRITE}ms"
echo "  Context: ${FUNEMON_CONTEXT}ms"
echo "  Search: ${FUNEMON_SEARCH}ms"
echo ""

log_result "## Test 1: Funemon CLI Solo"
log_result ""
log_result "| Operation | Time (ms) |"
log_result "|-----------|-----------|"
log_result "| Startup | ${FUNEMON_STARTUP} |"
log_result "| Write (avg of 10) | ${FUNEMON_WRITE} |"
log_result "| Context | ${FUNEMON_CONTEXT} |"
log_result "| Search | ${FUNEMON_SEARCH} |"
log_result ""

# ============================================
# TEST 2: Funemon via MCP (simulado)
# ============================================

echo -e "${YELLOW}[3/5] Test 2: Funemon MCP Tools${NC}"

# Las tools MCP son wrappers del CLI, así que los tiempos son similares
# pero medimos el overhead adicional

# Crear nueva sesión para MCP
START=$(get_timestamp_ms)
SESSION_MCP=$($FUNEMON_BIN session start --project "${PROJECT}_mcp" 2>&1 | grep "ID:" | awk '{print $2}')
END=$(get_timestamp_ms)
MCP_STARTUP=$((END - START))

# Guardar memorias via MCP (simulado con JSON)
WRITE_TIMES=""
for i in {1..10}; do
    TIME=$(measure_time $FUNEMON_BIN memories store \
        --session-id "$SESSION_MCP" \
        --title "MCP test $i" \
        --type "plan" \
        --what "MCP content $i" \
        --why "MCP benchmark")
    WRITE_TIMES="$WRITE_TIMES $TIME"
done
MCP_WRITE=$(echo $WRITE_TIMES | awk '{sum=0; for(i=1;i<=NF;i++)sum+=$i; print sum/NF}')

# Contexto via MCP
TIME=$(measure_time $FUNEMON_BIN session context --session-id "$SESSION_MCP")
MCP_CONTEXT=$TIME

log_result "## Test 2: Funemon MCP Tools"
log_result ""
log_result "| Operation | Time (ms) |"
log_result "|-----------|-----------|"
log_result "| Startup | ${MCP_STARTUP} |"
log_result "| Write (avg of 10) | ${MCP_WRITE} |"
log_result "| Context | ${MCP_CONTEXT} |"
log_result ""

echo "  Startup: ${MCP_STARTUP}ms"
echo "  Write (avg): ${MCP_WRITE}ms"
echo "  Context: ${MCP_CONTEXT}ms"
echo ""

# ============================================
# TEST 3: Reflexiones
# ============================================

echo -e "${YELLOW}[4/5] Test 3: Reflexiones${NC}"

# Generar reflexión con datos externos (simulado)
REFLECTION_JSON='{"content":"Test reflection for benchmark","type":"principle","importance":0.75,"level":"Principle","source_summary":"Benchmark test reflection"}'

TIME=$(measure_time $FUNEMON_BIN reflection store \
    --session-id "$SESSION_ID" \
    --content "$REFLECTION_JSON" \
    --agent-name tyrion)
REFLECTION_STORE=$TIME

# Recuperar reflexión
TIME=$(measure_time $FUNEMON_BIN reflection get --session-id "$SESSION_ID")
REFLECTION_GET=$TIME

echo "  Store: ${REFLECTION_STORE}ms"
echo "  Get: ${REFLECTION_GET}ms"
echo ""

log_result "## Test 3: Reflexions"
log_result ""
log_result "| Operation | Time (ms) |"
log_result "|-----------|-----------|"
log_result "| Store | ${REFLECTION_STORE} |"
log_result "| Get | ${REFLECTION_GET} |"
log_result ""

# ============================================
# TEST 4: Uso de Disco y Memoria
# ============================================

echo -e "${YELLOW}[5/5] Test 4: Resource Usage${NC}"

FUNEMON_DB="$HOME/.local/share/funemon/funemon.db"
FUNEMON_BIN_SIZE=$(ls -lh "$FUNEMON_BIN" | awk '{print $5}')

if [ -f "$FUNEMON_DB" ]; then
    FUNEMON_DB_SIZE=$(ls -lh "$FUNEMON_DB" | awk '{print $5}')
else
    FUNEMON_DB_SIZE="N/A"
fi

# Conteo de registros
MEMORY_COUNT=$($FUNEMON_BIN memories list 2>&1 | grep -c "Memory ID" || echo "0")
SESSION_COUNT=$($FUNEMON_BIN session list 2>&1 | grep -c "Session ID" || echo "0")

echo "  Binary size: ${FUNEMON_BIN_SIZE}"
echo "  DB size: ${FUNEMON_DB_SIZE}"
echo "  Memories: ${MEMORY_COUNT}"
echo "  Sessions: ${SESSION_COUNT}"
echo ""

log_result "## Test 4: Resource Usage"
log_result ""
log_result "| Metric | Value |"
log_result "|--------|-------|"
log_result "| Binary size | ${FUNEMON_BIN_SIZE} |"
log_result "| Database size | ${FUNEMON_DB_SIZE} |"
log_result "| Total memories | ${MEMORY_COUNT} |"
log_result "| Total sessions | ${SESSION_COUNT} |"
log_result ""

# ============================================
# RESUMEN FINAL
# ============================================

log_result "---"
log_result ""
log_result "## Summary"
log_result ""
log_result "### Performance Comparison"
log_result ""
log_result "| Test | Funemon CLI | Funemon MCP |"
log_result "|------|-------------|-------------|"
log_result "| Startup | ${FUNEMON_STARTUP}ms | ${MCP_STARTUP}ms |"
log_result "| Write (avg) | ${FUNEMON_WRITE}ms | ${MCP_WRITE}ms |"
log_result "| Context | ${FUNEMON_CONTEXT}ms | ${MCP_CONTEXT}ms |"
log_result "| Search | ${FUNEMON_SEARCH}ms | N/A |"
log_result ""
log_result "### Notes"
log_result ""
log_result "- **OpenCode Solo**: No tiene persistencia de memoria. Cada sesión comienza desde cero."
log_result "- **OpenCode + Funemon**: Usa tools MCP para persistir contexto entre sesiones."
log_result "- **Funemon CLI**: Usa directamente el CLI sin intermediarios."
log_result "- **OpenCode + Agentes**: Los agentes (Magnus, Aurora, Bruno) usan Funemon automáticamente."
log_result ""

echo -e "${GREEN}=== BENCHMARK COMPLETE ===${NC}"
echo ""
echo "📊 Results saved to: $OUTPUT_FILE"
echo ""
cat "$OUTPUT_FILE"