#!/usr/bin/env bash
set -euo pipefail

API="http://localhost:3000/api"

# ── helpers ──────────────────────────────────────────────────────────────────

usage() {
  echo "Usage: $0 <image-path> [options]"
  echo ""
  echo "Options:"
  echo "  -o FILE       Output file (default: gradient.jpg)"
  echo "  -p PARADIGM   linear | diagonal | radial | reflected  (default: linear)"
  echo "  -w WARP       none | ease_in | ease_out | ease_in_out | wave | zigzag  (default: none)"
  echo "  -W WIDTH      Image width in pixels (default: 1200)"
  echo "  -H HEIGHT     Image height in pixels (default: 160)"
  echo ""
  echo "Examples:"
  echo "  $0 photo.jpg"
  echo "  $0 photo.jpg -p radial -w ease_in_out"
  echo "  $0 photo.jpg -p diagonal -w wave -o wave.jpg"
  exit 1
}

step() { echo "▶ $*"; }
ok()   { echo "✓ $*"; }
fail() { echo "✗ $*" >&2; exit 1; }

# ── args ─────────────────────────────────────────────────────────────────────

[[ $# -lt 1 ]] && usage
IMAGE="$1"; shift

OUTPUT="gradient.jpg"
PARADIGM="linear"
WARP="none"
WIDTH=1200
HEIGHT=160

while getopts "o:p:w:W:H:" opt; do
  case $opt in
    o) OUTPUT="$OPTARG" ;;
    p) PARADIGM="$OPTARG" ;;
    w) WARP="$OPTARG" ;;
    W) WIDTH="$OPTARG" ;;
    H) HEIGHT="$OPTARG" ;;
    *) usage ;;
  esac
done

[[ -f "$IMAGE" ]] || fail "File not found: $IMAGE"

# ── check server ─────────────────────────────────────────────────────────────

step "Checking API server at $API ..."
curl -sf "$API/gradient/from-colors" -X POST \
  -H "Content-Type: application/json" \
  -d '{"colors":["#000000"],"steps":2}' > /dev/null \
  || fail "Server not reachable. Run:  cargo run -p api"
ok "Server is up"

# ── extract colors ────────────────────────────────────────────────────────────

step "Extracting dominant colors from $(basename "$IMAGE") ..."
RESPONSE=$(curl -sf -X POST "$API/image/extract-colors" \
  -F "image=@$IMAGE") \
  || fail "Color extraction failed"

COLORS=$(echo "$RESPONSE" | jq -c '.dominant_colors')
echo ""
echo "  Dominant colors:"
echo "$RESPONSE" | jq -r '.dominant_colors[]' | while read -r hex; do
  printf "    %s\n" "$hex"
done
echo ""

# ── render gradient ───────────────────────────────────────────────────────────

step "Rendering gradient (paradigm=$PARADIGM, warp=$WARP) → $OUTPUT ..."
curl -sf -X POST "$API/gradient/render" \
  -H "Content-Type: application/json" \
  -d "{
    \"colors\":   $COLORS,
    \"width\":    $WIDTH,
    \"height\":   $HEIGHT,
    \"paradigm\": \"$PARADIGM\",
    \"warp\":     \"$WARP\"
  }" \
  -o "$OUTPUT" \
  || fail "Gradient render failed"

ok "Saved to $OUTPUT"

# ── open ─────────────────────────────────────────────────────────────────────

if command -v open &>/dev/null; then
  open "$OUTPUT"
elif command -v xdg-open &>/dev/null; then
  xdg-open "$OUTPUT"
fi
