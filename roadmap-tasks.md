# Introduction 
> [!WARNING]
> Read this before you choose a task / topic.

This file contains some tasks that came to mind for porting Sherlock from GTK4
to GPUI. It is by far not a complete collection of everything though! I will
mark what people are working on with a [in progress] tag. That way, there wont
be dupplicate efforts on the same problem. Additionally, there will be a
[depends on] tag. This indicates that some tasks should not be touched if
another one is currently being worked on. For example, if a file should be
refactored, it would cause merge conflicts if someone else would add code to
it.


# Tasks

## [1] Improve Launcher Loading from `fallback.json`
[in progress (pretty much done for now)] 
This step includes several sub tasks:
- [x] Implement launcher loader 
- [ ] Improve the launcher logic (maybe macro for improved extesibility)
- [ ] Add bincode cache for the launcher logic with a trigger on original file change


## [2] Clean up `search_view.rs` 
[done for now]
The file is currently over 700 lines long. It should be refactored before
anything is added to it.

## [3] Add Filter/Sorter to `search_view`
[done for now]
Currently, the search bar takes inputs, but the input does not change the
sorting and filtering of the content.

## [4] Implement filtering logic for calculator
[done]
The calculators visibility is based on a couple of conditions. See original sherlock repo (src/ui/tiles/calc_tile.rs#based_show). This should be implmented similarly and if possible also improved.

## [5] Implement modes
The "mode" + <space> should switch sherlock modes again. Required for implentation of launcher alias

## [6] Variable input fields
[done]
This requires making the searchbar variable width (only taking as much space as needed) and inserting N new search fields next to it. Their values will also be required in the execution later, so we will need to pass them along somehow.

## [7] Context Menu
- [ ] Implement context menu indicator in the status bar
- [ ] Implement context menu keybind
- [ ] Implement context menu openining based on keybind and closing on <esc>

