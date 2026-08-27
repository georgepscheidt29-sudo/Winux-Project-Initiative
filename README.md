# Winux-Shell-Initiative
Repository of the Winux Shell Initiative.

## The Winux Shell is a project with the aim of learning Systems development and the Rust Programming Language,
### In it I intend on building a shell that runs Linux-like syntax on Windows OS, 

### The idea first appeared as I started learning more about using shell commands, I'd learn them on linux-sytems and then getting home to my windows 10 I'd have to keep looking up the "windows version" so I could practice/use them properly. 


### Currently implemented commands:

> | Command | Implementation                                                                                                                                              |
> |---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| | `pwd`   | Implemented via calling `current_dir()`                                                                                                                     |
| | `cd`    | Implemented via calling `set_curent_dir()`                                                                                                                  |
| | `ls`    | Implemented using `read_dir()`. with iterator to format the list, does not currently support arguments other than path (WIP)                                |
| | `clear` | Implemented via the standard `print!("\x1B[2J\x1B[1;1H")` method                                                                                            |
| | `exit`  | Currently sends a status 1 to the main loop, signaling it to break, will possibly change in the future when status comes to mean more to the overall system |


# Roadmap
> ##### These are not in any specific order apart from aproximated urgency and might go through updates, additions or removal at any time
--------------------------
## Near future

- [x] Refactor the system to build an error-friendly framework
- [ ] Divide commands by group and implement them as Structs, where each will have its exact command and args structure as well as handler implementation
- [ ] Implementation of simple argument parser and matcher
~~- [ ] Main arguments documentation for currently supported commands, probably in a new `args.rs` file~~ *(Scrapped idea, here as backlog of what once was. See second checklist entry)*
- [ ] Implementation of filesystem commands such as mkdir, rm, cat, touch, mv, cp

## Planned, but not an immediate concern

- [ ] Implementation and refactoring from the standard input-output crate to the rustyline crate
- [ ] Implementation of crossterm for keyboard events and terminal color (Just an idea, have to look into it and analyze if its actually what I have in mind)
- [ ] Further development of current result struct, with detailed status and dynamic result behavior