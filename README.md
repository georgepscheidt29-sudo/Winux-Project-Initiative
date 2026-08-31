# Winux-Shell-Initiative
Repository of the Winux Shell Initiative.

## The Winux Shell is a project with the aim of learning Systems development and the Rust Programming Language,
### In it I intend on building a shell that runs Linux-like syntax on Windows OS, 

### The idea first appeared as I started learning more about using shell commands, I'd learn them on linux-sytems and then getting home to my windows 10 I'd have to keep looking up the "windows version" so I could practice/use them properly. 


### Currently implemented commands:


| Command | Implementation                                                                                                                                                                   |
|:--------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `pwd`   | Implemented via calling `current_dir()`                                                                                                                                          |
| `cd`    | Implemented via calling `set_current_dir()` *(Fixed typo)*                                                                                                                       |
| `ls`    | Implemented using `read_dir()` with iterator to format the list, currently supports -a and -l arguments                                                                          |
| `clear` | Implemented via the standard `print!("\x1B[2J\x1B[1;1H")` method                                                                                                                 |
| `exit`  | Utilizes the RunResult enum, which is currently the backbone of the REPL system, any command other then exit sends a Continue, and the REPL waits for a RunResult::Exit to break |
| `mkdir` | Implemented via create_dir() fs method, with -p implemented via create_dir_all()                                                                                                 |
| `touch` | Implemented via File::create method for non-existent file names, and FileTimes to update timestamps on already existent files                                                    |
| `rm`    | Implemented via fs::remove, currently supports -i and -f args, -r and -R will probably be next                                                                                   |




# Roadmap
> ##### These are not in any specific order apart from aproximated urgency and might go through updates, additions or removal at any time
--------------------------
## Near future

- [x] Refactor the system to build an error-friendly framework
- [x] Divide commands by group and implement them as Structs, where each will have its exact command and ~~args structure~~ as well as handler implementation
~~- [x] Implementation of simple argument parser and matcher~~ *(Began implementation, however it was decided arguments will live inside execute logic, with special case for a --help, which will provide native documentation for all builtin commands)*
~~- [x] Main arguments documentation for currently supported commands, probably in a new `args.rs` file~~ *(Scrapped idea, here as backlog of what once was. See second checklist entry)*
- [ ] Implementation of filesystem commands such as mkdir, rm, cat, touch, mv, cp

## Planned, but not an immediate concern

- [ ] Implementation and refactoring from the standard input-output crate to the rustyline crate
- [ ] Implementation of crossterm for keyboard events and terminal color (Just an idea, have to look into it and analyze if its actually what I have in mind)
- [ ] Further development of current result struct, with detailed status and dynamic result behavior
- [ ] Look into which and pathsearch crates for future implementation