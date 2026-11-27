#!/usr/bin/env bash
set -e

: "${ATLASDB_URL:?Set ATLASDB_URL in Render}"

API_ADDR=127.0.0.1:3001 ./server/target/release/travaasa &
api_pid=$!

trap 'kill "$api_pid" ${web_pid:+"$web_pid"} 2>/dev/null || true' EXIT
trap 'exit 0' TERM INT

until curl -fsS --max-time 2 http://127.0.0.1:3001/health >/dev/null 2>&1; do
    if ! kill -0 "$api_pid" 2>/dev/null; then
        echo 'Travaasa API exited before it became ready.' >&2
        exit 1
    fi
    sleep 1
done

ORIGIN="${ORIGIN:-${RENDER_EXTERNAL_URL:-}}" API_URL=http://127.0.0.1:3001 node web/build &
web_pid=$!
wait -n "$api_pid" "$web_pid"
