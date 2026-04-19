#!/bin/bash
# latencia_iot.sh - Aplica delay 80ms jitter 20ms sobre la interfaz seleccionada
# Uso: sudo ./latencia_iot.sh [interfaz]

INTERFACE=${1:-eth0}

echo " [LATENCIA IoT] Aplicando delay 80ms ± 20ms jitter"
echo "================================================================"
echo "   Interfaz: $INTERFACE"

# Limpiar reglas existentes
sudo tc qdisc del dev "$INTERFACE" root 2>/dev/null

# Aplicar netem con delay y jitter
sudo tc qdisc add dev "$INTERFACE" root netem delay 80ms 20ms

echo ""
echo " Escenario aplicado"
echo ""
echo " Verificación:"
sudo tc qdisc show dev "$INTERFACE"
echo ""
echo " Prueba rápida de latencia:"
echo "   ping -c 5 8.8.8.8"
echo ""
echo " Para restaurar baseline: sudo ./baseline.sh $INTERFACE"
