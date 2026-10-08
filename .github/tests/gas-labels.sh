#!/usr/bin/env bash
set -euo pipefail

repo=$(cd "$(dirname "$0")/../.." && pwd)
workspace=$(mktemp -d)
trap 'rm -rf "$workspace"' EXIT
# Exercise the actual workflow step, including validation and report rendering.
awk '
  /- name: Compare operation gas/ { step = 1 }
  step && /        run: \|/ { script = 1; next }
  script && NF && !/^          / { exit }
  script { sub(/^          /, ""); print }
' "$repo/.github/workflows/gas-report.yml" > "$workspace/compare.sh"
test -s "$workspace/compare.sh"
cd "$workspace"
mkdir -p contracts/gas-operation-snapshots gas-baseline/contracts/gas-operation-snapshots
base=gas-baseline/contracts/gas-operation-snapshots/Measurements.json
head=contracts/gas-operation-snapshots/Measurements.json
export BASE_REF=next GITHUB_OUTPUT="$workspace/output"
printf '%s\n' '{"alpha":"100","beta":"200"}' > "$base"

check() {
  printf '{"alpha":"%s","beta":"%s"}\n' "$1" "$2" > "$head"
  : > "$GITHUB_OUTPUT"
  bash -euo pipefail compare.sh
  test "$(cat "$GITHUB_OUTPUT")" = "label=$3"
}
check 90 190 gas:measured-improvement
check 90 200 gas:measured-improvement
check 90 210 gas:mixed
check 110 190 gas:mixed
check 100 200 ''
check 110 210 ''
check 100 210 ''

for invalid in '{}' \
  '{"alpha":0,"beta":200}' \
  '{"alpha":1.5,"beta":200}' \
  '{"alpha":"bad","beta":200}' \
  'not JSON'; do
  printf '%s\n' "$invalid" > "$head"
  : > "$GITHUB_OUTPUT"
  if bash -euo pipefail compare.sh 2>/dev/null; then
    echo "Invalid snapshot accepted: $invalid" >&2
    exit 1
  fi
  test ! -s "$GITHUB_OUTPUT"
done
# A bad group after valid data must reject the entire comparison.
check 90 190 gas:measured-improvement
printf '%s\n' 'not JSON' > contracts/gas-operation-snapshots/zInvalid.json
: > "$GITHUB_OUTPUT"
if bash -euo pipefail compare.sh 2>/dev/null; then exit 1; fi
test ! -s "$GITHUB_OUTPUT"
rm contracts/gas-operation-snapshots/zInvalid.json
# Added/removed measurements and missing groups must not earn a label.
printf '%s\n' '{"alpha":90}' > "$head"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
check 90 190 gas:measured-improvement
printf '%s\n' '{"gamma":300}' > contracts/gas-operation-snapshots/AnotherGroup.json
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
printf '%s\n' '{"gamma":250}' > gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label=gas:mixed'
rm contracts/gas-operation-snapshots/AnotherGroup.json gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
rm "$head"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
rm "$base"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
# Mock only the API; execute both revision guards and label reconciliation unchanged.
export workspace GITHUB_REPOSITORY=example/repo PR_NUMBER=42 HEAD_SHA=head BASE_SHA=base
export GAS_LABEL=gas:measured-improvement
gh() {
  if [[ "$*" == *--method* ]]; then
    printf '%s\n' "$*" >> "$workspace/mutations"
  else
    cat "$workspace/pr.json"
  fi
}
export -f gh
for step in 'Clear managed labels for this comparison' 'Reconcile gas labels for current revisions'; do
  awk -v name="$step" '
    index($0, "- name: " name) { step = 1 }
    step && /        run: \|/ { script = 1; next }
    script && NF && !/^          / { exit }
    script { sub(/^          /, ""); print }
  ' "$repo/.github/workflows/gas-report.yml" > api-step.sh
  test -s api-step.sh
  for revisions in 'old base open' 'head old open' 'head base closed' 'head base open'; do
    read -r head_sha base_sha state <<< "$revisions"
    jq -n --arg head "$head_sha" --arg base "$base_sha" --arg state "$state" \
      '{head:{sha:$head},base:{sha:$base},state:$state,
        labels:[{name:"gas:mixed"},{name:"unrelated"}]}' > pr.json
    : > mutations
    : > "$GITHUB_OUTPUT"
    bash -euo pipefail api-step.sh
    if [[ "$revisions" == 'head base open' ]]; then
      grep -q 'DELETE.*gas%3Amixed' mutations
      if [[ "$step" == 'Reconcile gas labels for current revisions' ]]; then
        grep -q 'POST.*labels\[\]=gas:measured-improvement' mutations
      fi
      if grep -q unrelated mutations; then exit 1; fi
    else
      test ! -s mutations
      test ! -s "$GITHUB_OUTPUT"
    fi
  done
done
printf 'Gas label classification and revision checks passed\n'
