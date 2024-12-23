---
tags:
  - overview
---












---

```dataviewjs
dv.table(
	["Recents"], 
	dv.pages(`"${dv.current().file.folder}"`)
	.where(p =>
		   p.file.name != p.file.folder.split("/").pop()
	)
	.sort(page => page.file.mtime.toMillis(), "desc")
	.limit(10)
	.map(page => [
		page.file.link +
		"<span style='color: gray; font-size: 13px; margin-left: 2px; float: right'>" +
		page.file.mtime.toRelative() +
		"</span>"
		])
	);
```

