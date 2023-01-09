# rusty_pong

* 👾 Simple terminal based implementation of Pong
* 🚀 My first project in Rust!
* 📚 Uses (mostly) what I learned in the Udemy course [Ultimate Rust Crash Course
](https://www.udemy.com/course/ultimate-rust-crash-course/) by [@CleanCut (Nathan Stocks)](https://github.com/CleanCut)
* ‍💻️ Based on the [invaders example](https://github.com/CleanCut/invaders) repo but redone from scratch
* 🤓 I can recommend building the same to solidify learning after taking the course. The project is similar but different enough.

## Comments

### Where are the sounds coming from?
All sounds are taken from the [invaders example](https://github.com/CleanCut/invaders) projects because I am lazy and no fun.

### Why are the controls so bad? Why do player controls interrupt each other?
Handling of key presses and especially key releases are surprisingly difficult in terminals.
It is a rabbit hole which I did not jump fully in. There are some workarounds to make it work.
Using certain terminal implementations (e.g. [kitty](https://sw.kovidgoyal.net/kitty/)) and activating support in crossterm (see [here](https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html)) would make it possible, but I did not bother for this learning project.

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2.

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [license/APACHE](license/APACHE) and [license/MIT](license/MIT).