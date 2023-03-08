# TapOS
> Tap ain't POSIX

![](https://img.shields.io/github/actions/workflow/status/IsaacMarovitz/tap_os/build.yml?style=for-the-badge)

Experimental, in progress, operating system. Tap is based on a few core [design principles](https://isaacmarovitz.notion.site/TapOS-d00529f1697445fba412a0ebed03f3a8) that seek to modernise and streamline how operating systems work on modern devices.

Rusty from top to bottom 🦀

<img width="600" alt="Screenshot 2023-03-08 at 21 39 28" src="https://user-images.githubusercontent.com/42140194/223856339-f00d5f68-f2e4-4f1b-8146-c0fd5cfa7891.png">
It doesn't do much right now, but Rome wasn't built in a day.


<h2>Building:</h2>

1. Update rust: `rustup update`
2. Switch to nightly: `rustup override set nightly`
3. Add `llvm-tools-preview`: `rustup component add llvm-tools-preview`
4. Add x86_64 target: `rustup target add x86_64-unknown-none`
4. Build and run: `cargo run`
