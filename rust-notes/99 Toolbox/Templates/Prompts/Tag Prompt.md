Given content and existing tags list, generate relevant hierarchical tags. Rules:

Case 1 - Empty tags:

tags: []
or
tags:

Case 2 - No tags section:

other_field: value

Case 3 - No frontmatter at all:

Replace/add tags using this exact format:

- tag1
- tag2

Rules:

- For Case 1: Replace empty tags with properly indented list
- For Case 2: Add "tags:" with indented list after existing fields
- For Case 3: Create full frontmatter with tags
- if tags already exist, match indendation level with them.
- Use full paths (classes/comp-251)
- Keep indentation (two spaces)
- No duplicating 'tags:' line or --- markers
- No backticks / codeblocks in your response
- Focus on key concepts from frequency list, but create new tags if relevant
- ONLY PROVIDE THE TAG OUTPUT. nothing else.

Example output for all cases:

- classes/comp-251
- code/algorithms

Input: {{selected}}


Existing tag frequencies to consider when suggesting new tags:

#code: 243
#uni: 216
#uni/comp-303: 108
#code/practice: 54
#lab-research: 53
#code/machine-learning: 43
#uni/comp-302: 42
#code/machine-learning/ai-ethics: 41
#code/machine-learning/recommenders: 41
#code/design: 40
#lab-research/social-media: 40
#jobbing: 39
#uni/comp-251: 38
#jobbing/companies: 33
#world: 32
#code/languages: 26
#code/data-structures: 23
#world/ireland: 22
#world/ireland/bookstores: 22
#jobbing/companies/tiktok: 21
#quests: 20
#quests/trading-fours: 20
#code/concepts: 19
#uni/muar-392: 17
#code/data-structures/trees: 17
#code/networks: 17
#code/algorithms: 15
#uni/comp-303/chapter6: 14
#code/paradigms: 14
#overview: 14
#code/languages/ocaml: 13
#code/design/antipattern: 12
