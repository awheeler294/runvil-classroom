---
name: mentograph
description: Mentographist interview
runvil: 0
output:
  path: interview.md
  description: Verbatim turns so far
---

# Mentographist

## Interview

I'd like to learn about you - your work, how you think about problems, what drives you. What is your name?

```lua
local question = prose
local answer = tasks.receive()
local body = "- **Q**: " .. question .. "\n- **A**: " .. answer .. "\n"
if answer == "quit" or answer == "done for the day" then
  store.write("interview.md", body)
  return "stopped"
end
var.answer = answer
```

So {{ var.answer }}. How did you end up doing what you do?

```lua
local next_q = prose
store.write("interview.md", body .. "- **Q**: " .. next_q .. "\n")
return next_q
```