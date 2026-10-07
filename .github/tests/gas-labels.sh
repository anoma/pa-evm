#!/usr/bin/env bash
set -euo pipefail

repo=$(cd "$(dirname "$0")/../.." && pwd)
# PR fixtures must never receive repository secrets, including on manual fork comparisons.
if grep -Eq 'secrets[[:space:]]*[.[]' "$repo/.github/workflows/gas-report.yml"; then
  echo 'Gas measurement workflow must not reference repository secrets' >&2
  exit 1
fi
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
base=gas-baseline/contracts/gas-operation-snapshots/ExecutionGas.json
head=contracts/gas-operation-snapshots/ExecutionGas.json
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
  'not JSON' \
  '{"alpha":110,"beta":210} {"alpha":90,"beta":190}' \
  ''; do
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
# Missing execution measurements withhold labels; supporting groups do not affect them.
printf '%s\n' '{"alpha":90}' > "$head"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
check 90 190 gas:measured-improvement
printf '%s\n' '{"gamma":300}' > contracts/gas-operation-snapshots/AnotherGroup.json
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label=gas:measured-improvement'
printf '%s\n' '{"gamma":250}' > gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label=gas:measured-improvement'
rm contracts/gas-operation-snapshots/AnotherGroup.json gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
# Supporting savings/regressions cannot create, defeat, or mix execution labels.
printf '%s\n' '{"gamma":300}' > gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
printf '%s\n' '{"gamma":900}' > contracts/gas-operation-snapshots/AnotherGroup.json
check 90 190 gas:measured-improvement
grep -q '### Execution gas: improvement' contracts/operation-gas.md
grep -q 'AnotherGroup/gamma.*+600' contracts/operation-gas.md
check 90 210 gas:mixed
printf '%s\n' '{"gamma":1}' > contracts/gas-operation-snapshots/AnotherGroup.json
check 110 210 ''
grep -q '### Execution gas: regression' contracts/operation-gas.md
check 100 200 ''
grep -q '### Execution gas: unchanged' contracts/operation-gas.md
# Never infer execution scope from an arbitrary group or operation name.
mv "$base" gas-baseline/contracts/gas-operation-snapshots/Measurements.json
mv "$head" contracts/gas-operation-snapshots/Measurements.json
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
grep -q '### Execution gas: unclassified' contracts/operation-gas.md
mv gas-baseline/contracts/gas-operation-snapshots/Measurements.json "$base"
mv contracts/gas-operation-snapshots/Measurements.json "$head"
rm contracts/gas-operation-snapshots/AnotherGroup.json gas-baseline/contracts/gas-operation-snapshots/AnotherGroup.json
rm "$head"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
rm "$base"
: > "$GITHUB_OUTPUT"
bash -euo pipefail compare.sh
test "$(cat "$GITHUB_OUTPUT")" = 'label='
if grep -q '^| Operation |' contracts/operation-gas.md; then exit 1; fi
# Mock only the API; execute both revision guards and label reconciliation unchanged.
export workspace GITHUB_REPOSITORY=example/repo PR_NUMBER=42 HEAD_SHA=head BASE_SHA=base
export GAS_LABEL=gas:measured-improvement
export MEASURED_SHA=$(printf '%040d' 3)
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
# Execute the workflow's exact-revision resolver and local merge on a branch behind its base.
extract_step() {
  awk -v name="$1" '
    index($0, "- name: " name) { step = 1 }
    step && /        run: \|/ { script = 1; next }
    script && NF && !/^          / { exit }
    script { sub(/^          /, ""); print }
  ' "$repo/.github/workflows/gas-report.yml" > "$workspace/$2"
  test -s "$workspace/$2"
}
extract_step 'Resolve comparison revisions' resolve.sh
extract_step 'Prepare merged comparison' merge.sh
export GITHUB_EVENT_NAME=pull_request REPORT_SHA=$(printf '%040d' 1) BASE_SHA=$(printf '%040d' 2)
export GITHUB_ENV="$workspace/env"
: > "$GITHUB_OUTPUT"
: > "$GITHUB_ENV"
bash -euo pipefail "$workspace/resolve.sh"
grep -qx "head=$REPORT_SHA" "$GITHUB_OUTPUT"
grep -qx "base=$BASE_SHA" "$GITHUB_OUTPUT"
mkdir "$workspace/merge-fixture"
cd "$workspace/merge-fixture"
git init -q
git config user.name Test
git config user.email test@localhost
mkdir contracts
printf 'old\n' > contracts/value.txt
git add contracts
git commit -qm initial
initial=$(git rev-parse HEAD)
printf 'workflow\n' > workflow.txt
git add workflow.txt
git commit -qm workflow
export REPORT_SHA=$(git rev-parse HEAD)
git checkout -q --detach "$initial"
printf 'optimized\n' > contracts/value.txt
git commit -qam optimized
export BASE_SHA=$(git rev-parse HEAD)
git -c advice.detachedHead=false clone -q . gas-baseline
git checkout -q --detach "$REPORT_SHA"
: > "$GITHUB_OUTPUT"
bash -euo pipefail "$workspace/merge.sh" > /dev/null 2>&1
test "$(git rev-parse HEAD)" != "$REPORT_SHA"
git diff --exit-code "$BASE_SHA" HEAD -- contracts/value.txt
test -f workflow.txt
jq -e --arg base "$BASE_SHA" --arg head "$REPORT_SHA" --arg measured "$(git rev-parse HEAD)" \
  '. == {base:$base,head:$head,measured:$measured}' contracts/comparison-revisions.json > /dev/null
# A mismatched source checkout and a merge conflict must fail before exporting a measured revision.
: > "$GITHUB_OUTPUT"
if bash -euo pipefail "$workspace/merge.sh" > /dev/null 2>&1; then exit 1; fi
test ! -s "$GITHUB_OUTPUT"
git checkout -q --detach "$initial"
printf 'conflict\n' > contracts/value.txt
git commit -qam conflict
export REPORT_SHA=$(git rev-parse HEAD)
: > "$GITHUB_OUTPUT"
if bash -euo pipefail "$workspace/merge.sh" > /dev/null 2>&1; then exit 1; fi
test ! -s "$GITHUB_OUTPUT"
printf 'Exact base/head merge provenance and branch-drift checks passed\n'
# Exercise artifact identity and size rendering, including name collisions and removals.
extract_step 'Compare contract sizes' sizes.sh
cd "$workspace"
mkdir -p contracts/out/Fixtures gas-baseline/contracts/out/Fixtures
artifact() {
  jq -n --arg source "$2" --arg code "$4" \
    '{metadata:{settings:{compilationTarget:{($source):"SameName"}}},deployedBytecode:{object:$code}}' > "$1/out/Fixtures/$3.json"
}
artifact gas-baseline/contracts src/Unchanged.sol unchanged 0x0000
artifact contracts src/Unchanged.sol unchanged 0x0000
artifact gas-baseline/contracts src/Growing.sol growing 0x00
artifact contracts src/Growing.sol growing 0x000000
artifact gas-baseline/contracts src/Removed.sol removed 0x00
artifact contracts src/New.sol new 0x00
artifact contracts test/Unchanged.sol test 0x0000
bash -euo pipefail "$workspace/sizes.sh"
grep -Fq '| `src/Unchanged.sol:SameName` | 2 | 2 | +0 | 24574 |' contracts/contract-size-diff.md
grep -Fq '| `src/Growing.sol:SameName` | 1 | 3 | +2 | 24573 |' contracts/contract-size-diff.md
grep -Fq '| `src/Removed.sol:SameName` | 1 | — | removed | — |' contracts/contract-size-diff.md
grep -Fq '| `src/New.sol:SameName` | — | 1 | new | 24575 |' contracts/contract-size-diff.md
if grep -q 'test/' contracts/contract-size-diff.md; then exit 1; fi
cp contracts/out/Fixtures/unchanged.json contracts/out/Fixtures/duplicate.json
if bash -euo pipefail "$workspace/sizes.sh" 2>/dev/null; then exit 1; fi
rm contracts/out/Fixtures/duplicate.json
artifact contracts src/Invalid.sol invalid 0x0
if bash -euo pipefail "$workspace/sizes.sh" 2>/dev/null; then exit 1; fi
printf 'Contract size identity, zero-change, growth, new/removed and invalid artifact checks passed\n'
extract_step 'Build gas report comment' render.sh

mkdir "$workspace/report-fixture"
cd "$workspace/report-fixture"
printf '1|1|+0|0.00|\n' > gas-summary.txt
printf '%s\n' '{"classification":"unclassified","complete":false,"execution":[],"supporting":[]}' > operation-comparison.json
export GITHUB_STEP_SUMMARY="$workspace/summary" GITHUB_SERVER_URL=https://github.com GITHUB_RUN_ID=123
long_name='CommitmentTreeTest:test_commitmentTreeSides_and_commitmentTreeZeros_reproduce_the_root_after_every_push()'
printf '| Test | Base | PR | Delta | Percent |\n| --- | --- | --- | --- | --- |\n' > gas-diff.txt
for i in {1..25}; do printf '| `%s%s` | 100 | 100 | +0 | 0.00%% |\n' "$long_name" "$i"; done >> gas-diff.txt
printf '| Contract | Base | PR | Delta | Margin |\n| --- | --- | --- | --- | --- |\n| `src/Unchanged.sol:Unchanged` | 100 | 100 | +0 | 24476 |\n| `src/Warning.sol:Warning` | 23000 | 23000 | +0 | 1576 ⚠️ |\n' > contract-size-diff.md
: > "$GITHUB_STEP_SUMMARY"
bash -euo pipefail "$workspace/render.sh"
grep -Fq "$BASE_SHA" "$GITHUB_STEP_SUMMARY"
grep -Fq "$REPORT_SHA" "$GITHUB_STEP_SUMMARY"
grep -Fq "${MEASURED_SHA::7}" "$GITHUB_STEP_SUMMARY"
grep -q '0 changed; 25 unchanged' "$GITHUB_STEP_SUMMARY"
grep -q 'Warning.*23000.*23000.*+0 ⚠️' "$GITHUB_STEP_SUMMARY"
if grep -q 'Unchanged\|CommitmentTreeTest\|per-function\|<details>' "$GITHUB_STEP_SUMMARY"; then exit 1; fi
grep -q '/actions/runs/123#artifacts' "$GITHUB_STEP_SUMMARY"
# Long changed identifiers are shortened; the original full table stays intact.
for i in {1..15}; do printf '| `%s%s` | 100 | 110 | +10 | 10.00%% |\n' "$long_name" "$i"; done >> gas-diff.txt
: > "$GITHUB_STEP_SUMMARY"
bash -euo pipefail "$workspace/render.sh"
grep -q 'Showing 10 of 15 changed rows' "$GITHUB_STEP_SUMMARY"
grep -Fq "$long_name" gas-diff.txt
if grep -Fq "$long_name" "$GITHUB_STEP_SUMMARY"; then exit 1; fi
grep -q 'Commitment Tree Test' "$GITHUB_STEP_SUMMARY"
# Execution regressions remain visible beyond the normal ten-row summary cap.
jq -n '{classification:"regression",complete:true,supporting:[],execution:
  [range(1;13)|{name:("ExecutionGas/regression_"+tostring),before:100,after:110,delta:10}]}' > operation-comparison.json
: > "$GITHUB_STEP_SUMMARY"
bash -euo pipefail "$workspace/render.sh"
grep -q 'regression 12' "$GITHUB_STEP_SUMMARY"
printf 'Report provenance, shortened identifiers, unchanged counts, row limits and regression/warning visibility checks passed\n'
# Missing sides stay in artifacts; the comment shows no unpaired values or empty sections.
printf '%s\n' '{"classification":"unclassified","complete":false,"execution":[],"supporting":[]}' > operation-comparison.json
printf '| Test | Base | PR | Delta | Percent |\n| --- | --- | --- | --- | --- |\n| `OnlyNew:test_case()` | — | 123 | new | |\n' > gas-diff.txt
printf '| Contract | Base | PR | Delta | Margin |\n| --- | --- | --- | --- | --- |\n| `src/New.sol:New` | — | 23000 | new | 1576 ⚠️ |\n| `src/Removed.sol:Removed` | 123 | — | removed | — |\n' > contract-size-diff.md
: > "$GITHUB_STEP_SUMMARY"
bash -euo pipefail "$workspace/render.sh"
if grep -Eq '### (Execution|Supporting|Test|Contract)|23000|1576|unclassified|no gas label|PR margin' "$GITHUB_STEP_SUMMARY"; then exit 1; fi
# Every displayed metric has target, PR and delta context; no standalone margin survives.
printf '| Contract | Base | PR | Delta | Margin |\n| --- | --- | --- | --- | --- |\n| `src/Warning.sol:Warning` | 23000 | 23000 | +0 | 1576 ⚠️ |\n' > contract-size-diff.md
: > "$GITHUB_STEP_SUMMARY"
bash -euo pipefail "$workspace/render.sh"
grep -q '| Contract | Target bytes | PR bytes | Δ bytes |' "$GITHUB_STEP_SUMMARY"
if grep -q '1576\|PR margin' "$GITHUB_STEP_SUMMARY"; then exit 1; fi
printf 'Paired-only comment, no-fixture and no-baseline checks passed\n'
