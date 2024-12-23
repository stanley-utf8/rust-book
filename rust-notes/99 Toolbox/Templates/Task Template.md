```dataview
TASK
FROM #overview and <% tp.file.cursor(1) %>
WHERE !completed
SORT origin.file.mtime DESC, text ASC
```