# cosmos

`cosmos` is a cellular automata simulator.

Currently, it supports a version of the [Ulam-Warburton automaton](https://en.wikipedia.org/wiki/Ulam%E2%80%93Warburton_automaton) that I created. 
It has similar rules to the core Ulam Warburton automaton, but it also has a death timer that forces active cells to die and become inactive after being on for a configurable duration. 
Once a cell is off, it must wait a configurable duration before it can turn back on. This makes more interesting patterns than the base Ulam Warburton automaton. 

## TODO
- [x] Implement Advanced Ulam Warburton
- [x] Make Bevy preview
- [ ] Make expandable 2D grid data structure
- [ ] Make Bevy screen interactive
- [ ] Create GIF exporter
- [ ] Characterize the number of on cells for each type of the advanced UW automaton
- [ ] Add generic rule input version of CA
- [ ] Add documentation and pretty pictures