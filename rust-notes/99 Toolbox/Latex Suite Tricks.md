---
tags:
  - uni/comp-251
---
You can now run JavaScript code in snippets! To do this, make your `replacement` a function. For example, the snippet

```ts
{trigger: "date", replacement: () => (new Date().toDateString()), options: "t"},
```

will expand "dateTab" to the current date, such as "Mon Jan 15 2023".

Function snippets work with regex and visual snippets as well.

For [**regex** snippets](https://github.com/artisticat1/obsidian-latex-suite/releases#regex-snippets), Latex Suite will pass in the `RegExpExecArray` returned by the matching regular expression to your replacement function. This lets you access the value of capture groups inside your function. For example, the regex snippet

```ts
{trigger: /iden(\d)/, replacement: (match) => {
    const n = match[1];

    let arr = [];
    for (let j = 0; j < n; j++) {
        arr[j] = [];
        for (let i = 0; i < n; i++) {
            arr[j][i] = (i === j) ? 1 : 0;
        }
    }

    let output = arr.map(el => el.join(" & ")).join(" \\\\\n");
    output = `\\begin{pmatrix}\n${output}\n\\end{pmatrix}`;
    return output;
}, options: "mA"},
```


### Custom Regex


```
{
  trigger: /(.*)>>/,
  replacement: (match) => {
    return `- [ ] #follow-up ${match[1]}`;
  },
  options: "trA",
  priority: 1
},

// match one populates capture group
```

