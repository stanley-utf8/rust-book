
---

~~~dataviewjs
const file = app.workspace.getActiveFile();
const tags = app.metadataCache.getFileCache(file)?.tags?.map(a => a.tag);
if (tags) {
	const tagSet = new Set(tags);
	const tagArray = [];
	for (let tag of tagSet) {
		tagArray.push([tag, tags?.filter((b) => (b === tag))?.length]);
	}
	dv.table(["Tag", "\\# of occurences"], tagArray);
} else {
	dv.span("No tags");
}
~~~

```dataview
LIST
length(rows)
WHERE tags
FLATTEN file.tags as tags
GROUP BY tags
SORT length(rows) desc
LIMIT 10
```
