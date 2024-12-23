
```dataview
TASK
FROM #overview 
WHERE !completed
GROUP BY link as origin
SORT origin.file.mtime DESC, text ASC
```
^reqrdb


