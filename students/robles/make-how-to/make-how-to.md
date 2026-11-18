---
name: make-how-to
description: Manual of person
runvil: 0

args:
  type: object
  properties:
    name: { type: string, description: The name to research }
    text: { type: string, description: The text to base on }
  required: [name,text]

---

# Make How to
Make How-To turns a named person's written record into a how-to manual that teaches how they think. It searches the sources you point it at for that person's own words, keeps only the passages that carry transferable knowledge, distills each into a rule, and groups the rules into a reference a human or a model can load and apply. The tool names no source of its own; the operator says where to look, and web search is the only fallback. The finished how-to manual reads like a set of directives, each an imperative followed by the consequence that justifies it, grouped into themes that emerge from the evidence rather than a structure decided in advance.

# Gather

Find this person’s own words: {{ args.name }} in the following text and pull out only the person's own words that teach how they think, quoted: {{ args.text }} 


```lua
messages:user(prose)
models.loop()
store.write('notes.md', messages[#messages].content)
```

# Distill 

```lua
notes = store.read("notes.md")
```

Turn these notes into rules: each one an instruction and why it matters, grouped by theme.
{{ notes }}

```lua
return models.infer(prose)

```