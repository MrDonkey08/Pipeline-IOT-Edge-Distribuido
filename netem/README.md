# Módulo NetEm - Simulación de Red Degradada

##  Descripción General

Este módulo contiene los scripts de simulación de condiciones de red adversas utilizando `tc netem` (Traffic Control Network Emulator). Permite probar el comportamiento del pipeline IoT/Edge bajo diferentes escenarios de degradación de red, cumpliendo con los requisitos del Proyecto 2 de IL355.

Los scripts están diseñados para aplicarse sobre la interfaz de red especificada y son completamente reversibles mediante el script `baseline.sh`.

##  Estructura de Archivos
netem/  
├── baseline.sh # Restaura red a estado limpio  
├── latencia_iot.sh # Aplica delay 80ms ± 20ms jitter  
├── perdida_paquetes.sh # Aplica pérdida de paquetes del 8%  
├── enlace_limitado.sh # Limita ancho de banda a 512kbit + 50ms delay  
├── falla_nodo.sh # Detiene un contenedor edge específico  
├── status.sh # Muestra estado actual de reglas tc  
└── README.md # Este archivo  


##  Requisitos Previos

| Requisito | Versión | Comando de Instalación |
|-----------|---------|------------------------|
| Linux Kernel | 4.x+ | Incluido en distribución |
| iproute2 | Cualquiera | `sudo apt install iproute2` |
| iperf3 | 3.x+ | `sudo apt install iperf3` |
| Docker | 20.10+ | [docker.com](https://docker.com) |
| Bash | 4.0+ | Incluido por defecto |
| Permisos sudo | - | Requerido para modificar reglas tc |

### Verificación de Requisitos

```bash
# Verificar que tc está disponible
tc -V

# Verificar iperf3
iperf3 -v

# Verificar Docker
docker --version

# Verificar permisos sudo
sudo -v
```
///readme incompleto///
