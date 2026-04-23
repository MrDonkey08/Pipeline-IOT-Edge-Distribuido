#!/bin/bash
# status.sh - Muestra el estado actual de tc qdisc en todas las interfaces relevantes
# Uso: sudo ./status.sh

echo " ====== ESTADO DE TRAFFIC CONTROL (tc) ======"
echo "================================================================"
echo ""

# Detectar interfaces activas automaticamente
INTERFACES=$(ip -o link show | awk -F': ' '{print $2}' | grep -v "lo")

for iface in $INTERFACES; do
	# Verificar si la interfaz tiene IP asignada
	IP_ADDR=$(ip -4 addr show dev "$iface" 2> /dev/null | grep inet | awk '{print $2}' | head -1)

	if [ -n "$IP_ADDR" ]; then
		echo "🔹 Interfaz: $iface ($IP_ADDR)"
	else
		echo "🔹 Interfaz: $iface (sin IP)"
	fi

	QDISC=$(sudo tc qdisc show dev "$iface" 2> /dev/null)
	if [ -n "$QDISC" ]; then
		echo "$QDISC" | while read line; do
			echo "   $line"
		done
	else
		echo "    (sin reglas tc activas - red limpia)"
	fi
	echo ""
done

echo "================================================================"
echo " Comandos útiles:"
echo "   sudo ./baseline.sh <interfaz>     # Limpiar reglas"
echo "   sudo ./latencia_iot.sh <interfaz> # Aplicar latencia IoT"
echo "   ping -c 5 8.8.8.8                 # Probar latencia"
