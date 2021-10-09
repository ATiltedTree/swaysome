# swaysome

Make sway workspaces work on a per-screen basis.

# Usage

- `swaysome focus <NAME>` will focus the workspace `<NAME>` on the current screen.
- `swaysome move <NAME>` will move the current window to the workspace `<NAME>` on the current screen.
- `swaysome init <NAME>` will create a workspace `<NAME>` on every screen.
- `swaysome help` will print a help message.

Workspaces will be called `<SCREEN_INDEX><NAME>`, e.g. workspace `1` will be called `01` on the 0th screen and `11` on the first screen.

# Credits

Based off: https://git.hya.sk/skia/swaysome


## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.
