#### TODO 

##### Core
- [x] List names of files in current dir
- [x] List names of files in specific dir
- [x] Accept multiple arguments and list each with labels
- [] Sort names alphabetically by default

##### Options
- [] -h, --help List a help page
- [] -l List all files with their mode, owner, group, size, modification time
- [] -F Show file indicators (/ for directories, * for executables, etc.)
- [x] -a Include hidden files (Files starting with ".")

##### Other
- [] Load colors from LS_COLORS environment variable, falling back to default if not set

###### Default colors 

[Source](https://itsfoss.com/ls-color-output/)
| Filetype | Color | ANSI Escape Sequence |
| ---- | --- | --- |
| Normal | White | |
| Directory| Bold blue | |
| Symlink | Bold cyan | |
| Broken symlink | Red w/ black background | |
| Executable | Bold red | |
