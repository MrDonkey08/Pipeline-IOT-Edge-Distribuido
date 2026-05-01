#!/bin/bash
echo "=== COORDINATOR NODE (Docker) ==="
echo "LISTEN_PORT: ${LISTEN_PORT}"
echo "HTTP_PORT: ${HTTP_PORT}"
echo "================================"
exec /app/coordinator
