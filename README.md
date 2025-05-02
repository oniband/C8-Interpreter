# C8-Interpreter
This is my attempt at a Chip 8 emulator! It isn't currently feature complete (It's missing timers and sound) but I'm proud of where I got to regardless.
I've paused work on it for now whilst I go and get some more experience, but I hope to get back to this eventually.
This project really helped me get back into programming full time.

# Dependencies
You will need cargo(ideally rustup) installed to compile the program.

It only uses [raylib-rs](https://github.com/raylib-rs/raylib-rs) as a build dependency which has various requirements for each platform:
## Linux
- CMake
- Clang
## Windows
- Cmake
- mingw(gcc and tools)

# Building and running
I'm not entirely sure how to compile things for MacOS as I don't have access to a machine running it. The dependencies should be the same as linux.
## Build
### Linux
```
cargo build --release
```
### Windows
```
rustup target add x86_64-pc-windows-gnu 
```
```
cargo build --release --target x86_64-pc-windows-gnu 
```
-----
After building, you can find the program in `./target/release/Chip-8_Interpreter`.

# Running
Run the program from your terminal and pass it path to the ROM you'd like to run.
That's it :).

# Thanks
[Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) - For making an incredibly approachable guide on building a Chip-8 Emulator. This article inspired the whole project

[Tim Franssen](https://github.com/Timendus) - For making an [incredible suite of tests and resources](https://github.com/Timendus/chip8-test-suite). I would have been very lost without it.
