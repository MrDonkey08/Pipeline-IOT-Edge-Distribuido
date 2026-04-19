#!/bin/bash
# falla_nodo.sh - Detiene un contenedor edge específico
# Uso: ./falla_nodo.sh <nombre_contenedor>

echo " [FALLA DE NODO] Simulando caída de contenedor"
echo "================================================================"

if [ -z "$1" ]; then
    echo " Error: Debe especificar el nombre del contenedor"
    echo ""
    echo "Uso: $0 <nombre_contenedor>"
    echo ""
    echo "Contenedores actualmente corriendo:"
    docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Image}}' 2>/dev/null || echo "   (ninguno)"
    exit 1
fi

CONTAINER=$1

# Verificar si el contenedor existe y está corriendo
if docker ps --format '{{.Names}}' | grep -q "^${CONTAINER}$"; then
    echo " Deteniendo contenedor: $CONTAINER"
    docker stop "$CONTAINER"
    echo ""
    echo " Contenedor $CONTAINER detenido"
    echo ""
    echo " Para recuperar el nodo (simular reconexión):"
    echo "   docker start $CONTAINER"
    echo ""
    echo " Estado actual de contenedores:"
    docker ps -a --format 'table {{.Names}}\t{{.Status}}' | head -5
else
    echo " Contenedor '$CONTAINER' no está corriendo"
    echo ""
    echo "Contenedores disponibles:"
    docker ps -a --format 'table {{.Names}}\t{{.Status}}' 2>/dev/null || echo "   (ninguno)"
fi
