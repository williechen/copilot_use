#!/usr/bin/env bash
# Local or CI only: requires Docker Compose services already started and safe test credentials.
set -euo pipefail
: "${CHAT_API_TOKEN:?missing CHAT_API_TOKEN}"
: "${ANALYTICS_API_TOKEN:?missing ANALYTICS_API_TOKEN}"
CHAT_URL="${CHAT_URL:-http://127.0.0.1:3001}"
ANALYTICS_URL="${ANALYTICS_URL:-http://127.0.0.1:3100}"
for _ in $(seq 1 60); do
  if curl -fsS "${CHAT_URL}/healthz" >/dev/null &&
     curl -fsS "${ANALYTICS_URL}/healthz" >/dev/null; then break; fi
  sleep 2
done
curl -fsS "${CHAT_URL}/healthz" >/dev/null
curl -fsS "${ANALYTICS_URL}/healthz" >/dev/null
ROOM_ID="smoke-$(date +%s)-$RANDOM"
curl -fsS -X POST "${CHAT_URL}/api/v1/rooms" \
  -H "Authorization: Bearer ${CHAT_API_TOKEN}" -H "Content-Type: application/json" \
  -d "{\"id\":\"${ROOM_ID}\",\"name\":\"smoke-room\"}" >/dev/null
curl -fsS -X POST "${CHAT_URL}/api/v1/messages" \
  -H "Authorization: Bearer ${CHAT_API_TOKEN}" -H "Content-Type: application/json" \
  -d "{\"room_id\":\"${ROOM_ID}\",\"content\":\"smoke message\"}" >/dev/null
curl -fsS "${CHAT_URL}/api/v1/messages?room_id=${ROOM_ID}" \
  -H "Authorization: Bearer ${CHAT_API_TOKEN}" | python3 -c '
import json,sys
values=json.load(sys.stdin)
assert any(x["content"]=="smoke message" for x in values)
'
# Poll until dispatcher has delivered the durable event to Analytics.
for _ in $(seq 1 30); do
  if curl -fsS "${ANALYTICS_URL}/api/v1/events/summary" \
      -H "Authorization: Bearer ${ANALYTICS_API_TOKEN}" |
     python3 -c '
import json,sys
rows=json.load(sys.stdin)
assert any(x["source_app"]=="web_chat" and x["event_type"]=="message.persisted" and x["total"]>0 for x in rows)
' 2>/dev/null; then
    echo "PASS: Chat persisted message -> transactional outbox -> Analytics ingest/aggregation"
    exit 0
  fi
  sleep 2
done
echo "FAIL: durable Chat/Analytics event not observed" >&2
exit 1
