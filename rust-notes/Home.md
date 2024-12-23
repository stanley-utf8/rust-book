---

kanban-plugin: board
cssclasses:
  - no-scrollbar
  - hide-status
  - minimal-kanban

---

## [[Recents|quick list]]

- [ ] ![[green.webp]]
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


## #university

- [ ] ![[phone booth 1.webp]]
- [ ] [[Comp 303]]
- [ ] [[Comp 302]]
- [ ] [[Comp 251]]
- [ ] [[Math 323]]
- [ ] [[Muar 392]]


## #goals

- [ ] **2025**
	- build things
		- software / hardware
		- optimize for maintainability in the things I create
		- value true & learned knowledge
	- make music
		- produce, save up for gear, derive art and songs from inspiration
	- write & read
		- verse, journal, essay the things you build
		- dedicate to consistent, habitual reading (academically & personally)
	- take care of yourself & spend like you are your only source of income
		- workout, eat & sleep healthy – hit protein goals
		- budget consistently, cook from home
		- spending is ok, but realize the other ends where the money / lack thereof will eventually reveal
	- reach out more
		- I know as I get older the time I have with loved ones get smaller and smaller & the windows in which I have to see them diminish alongside
		- maximize your time with Toby and Archie
		- tell your parents how much you love and appreciate them
		- be more selfless in your words & let these show through your actions
	- define yourself around invaluableness & create an indispensable nature in your work
		- make these first impressions what carry you 
		- meet others and talk to those you respect and want advice from
	- explore
		- I live in a great city with so much to do, and I want to make the most out of it
		- go to new places and do new things


## [[Tasks|overview]]

- [ ] > [!class] [[Comp 251]]
	> 
	> ```dataview
	> TASK
	> FROM #overview and #classes/comp-251 
	> WHERE !completed
	> ```
- [ ] > [!class] [[02 Uni]]
	> ```dataview
	> TASK
	> FROM "02 Uni/02 Uni" and #overview and #uni
	> WHERE !completed
	> ```


## [[Follow Ups|follow ups]] & #review

- [ ] ![[Follow Ups#^ts6zqr]]


## #work



## collage

- [ ] ![[outlook.webp]]
- [ ] ![[booth 2.webp]]


***

## Archive

- [ ] ![[Tasks#^reqrdb]]

%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false,false,false,true,false,true,true],"hide-card-count":true,"new-line-trigger":"enter","show-add-list":true,"show-archive-all":false,"show-view-as-markdown":false,"show-set-view":true,"show-checkboxes":false,"lane-width":430,"show-board-settings":true,"full-list-lane-width":true,"link-date-to-daily-note":true,"inline-metadata-position":"body","move-tags":false}
```
%%