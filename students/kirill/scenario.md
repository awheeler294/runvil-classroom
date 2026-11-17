# Scenario: Office Coffee

1. ./run.sh
2. ./run.sh prompts/17.11.2026-nash.md "Office Coffee"
3. ./run.sh prompts/17.11.2026-nash.md "add player Alice"
4. ./run.sh prompts/17.11.2026-nash.md "add player Bob"
5. ./run.sh prompts/17.11.2026-nash.md "add game Coffee Break"
6. ./run.sh prompts/17.11.2026-nash.md "show state"

cd ~/work/cppalliance/runvil-classroom/students/kirill
url=$(~/work/cppalliance/runvil-workshop/target/debug/runvil-gateway --print-url)
export RUNVIL_GATEWAY_URL="${url%%/auth*}/v1" RUNVIL_GATEWAY_API_KEY="${url#*key=}" RUNNER_MODEL=Ornith-1.5-9B-Q4_K_M

7. ./run.sh prompts/17.11.2026-nash.md "Alice and Bob share the office coffee machine. After a pot runs out, each either Cleans the machine or Leaves It. They play Coffee Break, which starts when a pot runs out: each decides alone, then whoever wants the next pot finds the machine clean or dirty."
8. ./run.sh prompts/17.11.2026-nash.md "Payoffs for Coffee Break: if both clean, each gets 1. If one cleans and the other leaves it, the cleaner gets -1 and the other gets 2. If both leave it, each gets 0."
9. ./run.sh prompts/17.11.2026-nash.md "Also, remind me to buy filters tomorrow."
10. ./run.sh prompts/17.11.2026-nash.md "show state"
11. ./run.sh prompts/17.11.2026-nash.md "remove player Bob"
12. ./run.sh prompts/17.11.2026-nash.md "test me"
13. ./run.sh prompts/17.11.2026-nash.md "show state"
