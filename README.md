# MelonGB

**MelonGB** is a Gameboy (DMG) and Gameboy Color (CGB) Emulator written in Rust. Try out the demo [HERE](https://gordnzhou.github.io/melon-gb/)! Or
jump to [Installation](#installation) if you're interested in running it locally. 

This emulator is expected to be M-cycle accurate, apart from a few edge cases from some games that I've tested. Jump to [Passings Tests](#passing-tests) for a list of passing test ROMS and future TODOs. Some notable features include:
- Support for games that use MBC 1,2,3,5
- Saving/loading data for battery-backed games 
- Ability to import/export save data
- In-sync audio emulation for all 4 channels   

## Screenshots
|       |  |
| ----------- | ----------- |
| ![pokemongold](images/pokemongold.png) | ![pokemonred](images/pokemonred.png) |
| ![shantae](images/shantae.png)  | ![drmario](images/drmario.png) |
| ![cgb-acid2](images/cgb-acid2.png)  |        |

## Installation 
**(NOTE: Installation has only been tested on Mac)**
Before starting, make sure you have [Rust](https://www.rust-lang.org/tools/install) and [SDL2](https://wiki.libsdl.org/SDL2/Installation) installed and properly linked. 
1. Clone the repository
1. Add your ROM files to the `/roms` folder
1. (Optional) Edit the constants in `src/config.rs` 
    - You can add your Gameboy and Gameboy Color boot ROMs by specifying their ROM file path in `src/config.rs`
1. It `src/main.rs`, specify your ROM path and if you want run with the boot ROM, then run it. Enjoy!

### Passing Tests
- Blargg Tests
    - cpu_instrs
    - instr_timing
    - mem_timing
    - dmg_sound
    - cgb_sound
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2)
- [cgb-acid2](https://github.com/mattcurrie/cgb-acid2?tab=readme-ov-file)
- [Mooneye Test Suite](https://github.com/Gekkio/mooneye-test-suite/tree/main?tab=readme-ov-file)
    - **emulator-only**
        - all MBC tests
    - **acceptance**
        - oam_dma
            - basic.gb
            - reg_read.gb
        - interrupts
            - ie_push.gb
        - instr
            - daa.gb
    - **manual-only**
        - sprite_priority.gb

### Future TODOs
- Passing Mooneye Timing and PPU Tests
- Fixing edge cases in some GBC games




