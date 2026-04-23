#!/bin/bash
# baseline.sh - Elimina todas las reglas tc activas y restaura red limpia
# Uso: sudo ./baseline.sh [interfaz]

INTERFACE=${1:-eth0}

echo " [BASELINE] Limpiando reglas tc en interfaz: $INTERFACE"
echo "================================================================"

# Eliminar todas las reglas qdisc
sudo tc qdisc del dev "$INTERFACE" root 2> /dev/null && echo " Reglas eliminadas" || echo " No habia reglas activas"

# Verificar estado
echo ""
echo " Estado actual de qdisc en $INTERFACE:"
sudo tc qdisc show dev "$INTERFACE" || echo "   (sin reglas tc activas)"

echo ""
echo " Baseline restaurado - red limpia"
echo " Verifica conectividad: ping -c 3 google.com"
