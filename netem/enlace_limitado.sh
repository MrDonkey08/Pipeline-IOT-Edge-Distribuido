#!/bin/bash
# enlace_limitado.sh - Aplica rate 512kbit delay 50ms sobre la interfaz
# Uso: sudo ./enlace_limitado.sh [interfaz]

INTERFACE=${1:-eth0}

echo " [ENLACE LIMITADO] Aplicando 512kbit + delay 50ms"
echo "================================================================"
echo "   Interfaz: $INTERFACE"

# Limpiar reglas existentes
sudo tc qdisc del dev "$INTERFACE" root 2>/dev/null

# Aplicar tbf (Token Bucket Filter) para limitar ancho de banda
sudo tc qdisc add dev "$INTERFACE" root handle 1: tbf rate 512kbit burst 1600 latency 50ms

# Añadir netem para el delay
sudo tc qdisc add dev "$INTERFACE" parent 1:1 handle 10: netem delay 50ms

echo ""
echo " Escenario aplicado"
echo ""
echo " Verificación:"
sudo tc qdisc show dev "$INTERFACE"
echo ""
echo " Prueba rápida de velocidad (requiere iperf3):"
echo "   iperf3 -c iperf3.velocityonline.net -p 5201"
echo ""
echo " Para restaurar baseline: sudo ./baseline.sh $INTERFACE"
