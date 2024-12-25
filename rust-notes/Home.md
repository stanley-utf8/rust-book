---

kanban-plugin: board
cssclasses:
  - no-scrollbar
  - hide-status
  - minimal-kanban

---

## [[Recents|quick list]]

- [ ] - re [[Recents|Recents]]
	```dataviewjs
	dv.table(
		[], // column name
		dv.pages()
		// excluded files
		.where(page => page.file.name != "Home" && page.file.name != "Recents"
		&& page.file.name != "Tasks"
		// sort by recently updated
		// && page.file.name != page.file.folder.split("/").pop()
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
- [ ] - ! Important


## [[Follow Ups|follow ups]] & #review

- [ ] ![[Follow Ups#^ts6zqr]]


***

## Archive

- [ ] ![[Tasks#^reqrdb]]

%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false],"hide-card-count":true,"new-line-trigger":"enter","show-add-list":true,"show-archive-all":false,"show-view-as-markdown":false,"show-set-view":true,"show-checkboxes":false,"lane-width":430,"show-board-settings":true,"full-list-lane-width":true,"link-date-to-daily-note":true,"inline-metadata-position":"body","move-tags":false}
```
%%