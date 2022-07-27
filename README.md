# CellularAutomataRust
Learning Rust by implementing different Cellular Automata

# How to run
1. Clone the repository
2. Run ```cargo build --release```
3. Go into the target/release folder
4. Run the exe you want

# Project structure
The project consists mostly of a library, ```cell_engine```, implementing visualization, controls and other stuff common to cellular automata. 
This library is then used in multiple different cellular automata binaries, which have to implement game specific stuff such as update rules and cell varieties.
To build a new cellular automata with the library, only one function has to be called with a structure implementing some traits exposed by a library. 

# Cellular Automata 
1. Wireworld
2. Game of life
3. Langton's Ant
