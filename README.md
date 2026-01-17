# Palette Mapper 🎨

> Make any image fit your color scheme 

## What is this?

Palette Mapper is split into 3 different things: 

- The core library (located at `lib/`). This holds the implementation for taking a color palette and mapping it onto an image. 

- An accompanying CLI (located at `cli/`). This allows you to interact with the library from your terminal.

- The website (located at `web/`). This enables you to use the library from the comfort of your browser. 

## 🚧 TODO 🚧

<!-- TODO: See below -->

- [ ] Add testing to `lib` and `cli`

    - [ ] Benchmarking with hyperfine for `cli`

    - [x] Benchmarking with criterion for `lib`

    - [x] Snapshot testing for `lib`

- [ ] Create `web`

- [ ] Add pre-defined palletes 

- [ ] Implement more distance algorithms

    - [x] CIE Hybrid

    - [ ] CIEDE2000

    - [x] CIE76 LAB

    - [x] Manhattan Distance

- [ ] Add examples

- [ ] *(Possibly)* Expand options for cli

    - [ ] Blurs

    - [ ] Batch processing

    - [ ] Lightening 

## 🖼️ Examples 🖼️

...

## 💝 Acknowledgments 💝

- Thanks to the creators of [image](https://crates.io/crates/image) for their amazing library, without which this project would not have been possible.

## 📜 License 📜

This project is dual licensed under `MIT` and `Apache-2.0`

