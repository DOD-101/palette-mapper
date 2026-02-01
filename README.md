# <img src="./web/static/favicon.svg" style="height:1em;"> Palette Mapper

> Make any image fit your color scheme

## What is this?

Palette Mapper is split into 3 different things:

- The core library (located at `lib/`). This holds the implementation for taking a color palette and mapping it onto an image.

- An accompanying CLI (located at `cli/`). This allows you to interact with the library from your terminal.

- The website (located at `web/`). This enables you to use the library from the comfort of your browser.

## 🚧 TODO 🚧

### 0.1 release

<!-- TODO: See below -->

- [ ] `web`

    - [ ] Notifications (see `./web/src/lib/ui_notification.ts`)

    - [ ] Make it look better

        - [ ] Change color scheme (dependent on selected palette?)

        - [ ] Add grid to background

        - [ ] Show image output location

            - [ ] Before and after?

        - [ ] Allow toggling light and dark mode

    - [ ] Cleanup (remove stray `console.log`s, etc.)

- [ ] `cli`

    - [ ] completions (via clap-complete)

- [ ] Add examples

- [ ] Finish other todos

---

### Other (lower priority)

- [ ] Implement more distance algorithms

    - [ ] CIEDE2000

    - [x] CIE Hybrid

    - [x] CIE76 LAB

    - [x] Manhattan Distance

- [ ] Add caching to the ci

- [ ] *(Possibly)* Expand options for cli

    - [ ] Blurs

    - [ ] Batch processing

    - [ ] Lightening

---

### Done

- [x] Add pre-defined palettes

- [x] Add checks for misspelling of palette

- [x] Create `web`

- [x] Add testing

    - [x] ~Benchmarking with hyperfine for `cli`~

    - [x] e2e testing for `web`

    - [x] CI

    - [x] Benchmarking with criterion for `lib`

    - [x] Snapshot testing for `lib`

## 🖼️ Examples 🖼️

...

## 💝 Acknowledgments 💝

- Thanks to the creators of [image](https://crates.io/crates/image) for their amazing library, without which this project would not have been possible.

- Thanks to [Tinted Theming](https://github.com/tinted-theming) for their great collection of schemes used to create the built-in palettes found under `./palettes/`

## 📜 License 📜

The palettes found in `./palettes/base16/` and `./palettes/base24/` are derived (using `./fetch-palettes.sh`) from [Tinted Theming](https://github.com/tinted-theming/schemes) and are licensed under `MIT`.

This project is dual licensed under `MIT` and `Apache-2.0`
