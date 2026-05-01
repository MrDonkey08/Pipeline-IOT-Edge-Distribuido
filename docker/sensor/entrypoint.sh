#!/bin/bash
echo "=== SENSOR NODE (Docker) ==="
echo "ID: ${SENSOR_ID}"
echo "EDGE: ${EDGE_HOST}:${EDGE_PORT}"
echo "INTERVAL: ${INTERVAL_MS}ms"
echo "============================"
exec /app/sensor
