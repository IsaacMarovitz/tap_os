# TapOS
> Tap ain't POSIX

Experimental, in progress, operating system. Tap is based on a few core [design principles](https://isaacmarovitz.notion.site/TapOS-d00529f1697445fba412a0ebed03f3a8) that seek to modernise and streamline how operating systems work on modern devices.

Rusty from top to bottom 🦀


**Building:**
1. Update rust: `rustup update`
2. Switch to nightly: `rustup override set nightly`
3. Add `llvm-tools-preview`: `rustup component add llvm-tools-preview`
4. Install `bootimage`: `cargo install bootimage`
5. Build and run: `cargo run`
