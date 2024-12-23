```dataviewjs
dv.table(
	[],
	dv.pages()
        .where(page => page.file.name != "Home" && page.file.name != "Recents"
        && page.file.name != "Tasks")
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
^bf09f4


---

```
dataview
TABLE durationformat(date(now) - file.mtime, "h'h' m'm'") + " ago" AS "Updated"
FROM ""
WHERE file.name != "Home"
SORT file.mtime DESC
LIMIT 15

dataviewjs
const showTime = true;
dv.table(
	showTime ? ["Recents", "Last modified"] : ["Recents"],
	dv.pages()
		.where(page => page.file.name != "Home")
		.sort(page => page.file.mtime.toMillis(), "desc")
		.limit(15)
		.map(page => showTime ? [page.file.link, page.file.mtime.toRelative()] : [page.file.link])
);
```
