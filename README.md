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

        - [x] Change color scheme (dependent on selected palette?)

        - [x] Add grid to background

        - [ ] Show image output location

            - [ ] Before and after?

        - [ ] Allow toggling light and dark mode

    - [ ] Cleanup (remove stray `console.log`s, etc.)

- [ ] Finish other todos

---

### Other (lower priority)

- `web`

    - [ ] allow clicking on image to bring up preview pop-up

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

- [x] Add examples

- [x] Add pre-defined palettes

- [x] Add checks for misspelling of palette

- [x] Create `web`

- [x] Add testing

    - [x] ~Benchmarking with hyperfine for `cli`~

    - [x] e2e testing for `web`

    - [x] CI

    - [x] Benchmarking with criterion for `lib`

    - [x] Snapshot testing for `lib`

- [x] `cli`

    - [x] completions (via clap-complete)

## 🖼️ Examples 🖼️

### Tokyo Rail

<details open>
    <summary>Images</summary>
    <img width=400 src="./assets/examples/tokyo_rail.jpg" alt="Tokyo Rail original">
    <img width=400 src="./assets/examples/tokyo_rail_GruvboxLight.jpg" alt="Tokyo Rail GruvboxLight">
    <img width=400 src="./assets/examples/tokyo_rail_Material.jpg" alt="Tokyo Rail Material">
    <img width=400 src="./assets/examples/tokyo_rail_Moonlight.jpg" alt="Tokyo Rail Moonlight">
    <img width=400 src="./assets/examples/tokyo_rail_OnedarkDark.jpg" alt="Tokyo Rail OnedarkDark">
    <img width=400 src="./assets/examples/tokyo_rail_RosePine.jpg" alt="Tokyo Rail RosePine">
    <img width=400 src="./assets/examples/tokyo_rail_TokyoNightMoon.jpg" alt="Tokyo Rail TokyoNightMoon">
    <img width=400 src="./assets/examples/tokyo_rail_TokyoNightStorm.jpg" alt="Tokyo Rail TokyoNightStorm">
</details>

### Chicago

<details open>
    <summary>Images</summary>
    <img width=400 src="./assets/examples/chicago.jpg" alt="Chigao original">
    <img width=400 src="./assets/examples/chicago_CatppuccinMocha.jpg" alt="Chigao CatppuccinMocha">
    <img width=400 src="./assets/examples/chicago_Darcula.jpg" alt="Chigao Darcula">
    <img width=400 src="./assets/examples/chicago_Elemental.jpg" alt="Chigao Elemental">
    <img width=400 src="./assets/examples/chicago_Moonlight.jpg" alt="Chigao Moonlight">
    <img width=400 src="./assets/examples/chicago_OneDark.jpg" alt="Chigao OneDark">
    <img width=400 src="./assets/examples/chicago_RosePineDawn.jpg" alt="Chigao RosePineDawn">
    <img width=400 src="./assets/examples/chicago_WildCherry.jpg" alt="Chigao WildCherry">
</details>

### NYC Subway

<details open>
    <summary>Images</summary>
    <img width=400 src="./assets/examples/nyc_subway.jpg" alt="NYC Subway Original">
    <img width=400 src="./assets/examples/nyc_subway_AyuDark.jpg" alt="NYC Subway AyuDark">
    <img width=400 src="./assets/examples/nyc_subway_CatppuccinMocha.jpg" alt="NYC Subway CatppuccinMocha">
    <img width=400 src="./assets/examples/nyc_subway_EverforestDarkMedium.jpg" alt="NYC Subway EverforestDarkMedium">
    <img width=400 src="./assets/examples/nyc_subway_GruvboxDarkHard.jpg" alt="NYC Subway GruvboxDarkHard">
    <img width=400 src="./assets/examples/nyc_subway_Seti.jpg" alt="NYC Subway Seti">
    <img width=400 src="./assets/examples/nyc_subway_Zenbones.jpg" alt="NYC Subway Zenbones">
    <img width=400 src="./assets/examples/nyc_subway_DarkMetalKhold.jpg" alt="NYC Subway DarkMetalKhold">
</details>

## CLI completions

We use [clap-complete's](https://docs.rs/clap_complete/latest/clap_complete/env/index.html) `unstable-dynamic` feature to generate completions.

To generate completions run (see clap-complete's docs for more info):

`echo "source <(COMPLETE=bash your_program)" >> ~/.bashrc`

## 💝 Acknowledgments 💝

- Thanks to the creators of [image](https://crates.io/crates/image) for their amazing library, without which this project would not have been possible.

- Thanks to [Tinted Theming](https://github.com/tinted-theming) for their great collection of schemes used to create the built-in palettes found under `./palettes/`

## 📜 License 📜

### Examples

- `./assets/examples/tokyo_rail.jpg` by [Nicholas Cole](https://www.flickr.com/photos/ncole458/) is licensed under [CC BY 2.0](https://creativecommons.org/licenses/by/2.0/). Source: https://www.flickr.com/photos/ncole458/4869138986/in/photostream/

- `./assets/examples/chicago.jpg` by [Sarthak Banga](https://www.pexels.com/@sarthak-banga-2150431485/) is in the Public Domain. Source: https://www.pexels.com/photo/snowy-chicago-twilight-skyline-with-lake-michigan-35613845/

- `./assets/examples/nyc_subway.jpg` by  [Sarthak Banga](https://www.pexels.com/@sarthak-banga-2150431485/) is in the Public Domain. Source: https://www.pexels.com/photo/night-train-passing-through-snowy-chicago-streets-35449810/



The palettes found in `./palettes/base16/` and `./palettes/base24/` are derived (using `./fetch-palettes.sh`) from [Tinted Theming](https://github.com/tinted-theming/schemes) and are licensed under `MIT`.

This project is dual licensed under `MIT` and `Apache-2.0`
