#!/bin/bash
# perdida_paquetes.sh - Aplica loss 8% sobre la interfaz seleccionada
# Uso: sudo ./perdida_paquetes.sh [interfaz]

INTERFACE=${1:-eth0}

echo " [PERDIDA DE PAQUETES] Aplicando loss 8%"
echo "================================================================"
echo "   Interfaz: $INTERFACE"

# Limpiar reglas existentes
sudo tc qdisc del dev "$INTERFACE" root 2>/dev/null

# Aplicar netem con pérdida de paquetes
sudo tc qdisc add dev "$INTERFACE" root netem loss 8%

echo ""
echo " Escenario aplicado"
echo ""
echo " Verificación:"
sudo tc qdisc show dev "$INTERFACE"
echo ""
echo " Prueba rápida de pérdida:"
echo "   ping -c 20 8.8.8.8  # Deberías ver ~8% de pérdida"
echo ""
echo " Para restaurar baseline: sudo ./baseline.sh $INTERFACE"
