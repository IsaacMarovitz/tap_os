# TapOS
> Tap ain't POSIX

![](https://img.shields.io/github/actions/workflow/status/IsaacMarovitz/tap_os/build.yml?style=for-the-badge)

Experimental operating system that does nothing. Rusty from top to bottom 🦀

<img width="600" alt="Screenshot 2023-03-08 at 21 39 28" src="https://user-images.githubusercontent.com/42140194/223856339-f00d5f68-f2e4-4f1b-8146-c0fd5cfa7891.png">

<h2>Building:</h2>

1. Update rust: `rustup update`
2. Switch to nightly: `rustup override set nightly`
3. Add `llvm-tools-preview`: `rustup component add llvm-tools-preview`
4. Add x86_64 target: `rustup target add x86_64-unknown-none`
4. Build and run: `cargo run`
