# CellularAutomataRust
Learning Rust by implementing different Cellular Automata

## How to run
1. Clone the repository
2. Run ```cargo build --release```
3. Go into the target/release folder
4. Run the exe you want

## Project structure
The project consists mostly of a library, ```cell_engine```, implementing visualization, controls and other stuff common to cellular automata. 
This library is then used in multiple different cellular automata binaries, which have to implement game specific stuff such as update rules and cell varieties.
To build a new cellular automata with the library, only one function has to be called with a structure implementing some traits exposed by a library. 

## Controls
- P: Toggle pause
- PageUp: Increase game speed
- PageDown: Decrease game speed
- Space: One game step forward
- Shift + PageUp: Increase visual decay rate
- Shift + PageDown: Decrease visual decay rate
- Clicking or dragging with the mouse toggles the cells under the cursor
  
## Cellular Automata 
1. Wireworld
    - Cell types: Wire, Electron Head, Electron Tail
    - Update rule: 
      1. A inert cell stays inert
      2. A wire becomes a electron head if one or two neighboring cells are electron heads
      3. A electron head becomes a electron tail
      4. A electron tail becomes a wire
2. Game of life
    - Cell types: Dead, Alive
    - Update rule:
        1. A dead cell becomes alive if 3 neighboring cells are alive
        2. A alive cell stays alive if 2 neighboring cells are alive
3. Langton's Ant
    - Cell types: Black, White
      - Both cell types can contain the unique "ant"
      - The ant can face in all four directions
      - The number of different states per cell is therefore 2*4 + 2 = 10
    - Update rule:
      1. The ant always flips the color of the square it is on
      2. The ant turns left and then moves forward if it's on a black square
      3. The ant turns right and then moves forward if it's on a black square
