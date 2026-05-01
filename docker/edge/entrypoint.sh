#!/bin/bash
echo "=== EDGE NODE (Docker) ==="
echo "ID: ${EDGE_ID}"
echo "COORDINATOR: ${COORDINATOR_HOST}:${COORDINATOR_PORT}"
echo "HTTP_PORT: ${HTTP_PORT}"
echo "=========================="
exec /app/edge
