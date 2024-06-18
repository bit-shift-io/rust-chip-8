# rust-chip-8

A chip-8 emulator written in rust.

![](https://austinmorlan.com/posts/chip8_emulator/media/test_opcode.png)

## ROMs

ROM's can be downloaded using ```download_roms.sh```

Additional sources:

    https://github.com/loktar00/chip8/tree/master/roms
    
## Install

Run ```install.sh``` to setup rust project

You need the vscode extensions:

        CodeLLDB
        rust-analyzer

## Development

### Manually

        cargo build
        cargo run

        cargo add XXX # to add a cargo package

### Vscode

    ctrl + shift + b - to run the tasks

    open in vscode then just hit debug!

## Benchmark/Testing

    cargo bench
    cargo test

## Tutorials & Links

https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
https://austinmorlan.com/posts/chip8_emulator/
    
## Troubleshooting

* Can't debug rust in vscode: https://stackoverflow.com/questions/77218022/why-is-my-debugger-in-vscode-not-working-with-rust-after-mac-update-to-sonoma-14
