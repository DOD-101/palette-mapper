//! CLI tool for `palette_mapper` lib.
use palette_mapper::{color_pallete, map_image_to_palette};

fn main() {
    let palette = color_pallete!(
        // Base tones
        [24, 24, 37],    // base
        [30, 30, 46],    // mantle
        [17, 17, 27],    // crust
        [205, 214, 244], // text
        [166, 173, 200], // subtext0
        [186, 194, 222], // subtext1
        [108, 112, 134], // overlay0
        [127, 132, 156], // overlay1
        [147, 153, 178], // overlay2
        [88, 91, 112],   // surface2
        [69, 71, 90],    // surface1
        [49, 50, 68],    // surface0
        // Accent colors
        [243, 139, 168], // red
        [235, 160, 172], // maroon
        [250, 179, 135], // peach
        [249, 226, 175], // yellow
        [166, 227, 161], // green
        [148, 226, 213], // teal
        [137, 220, 235], // sky
        [116, 199, 236], // sapphire
        [137, 180, 250], // blue
        [180, 190, 254], // lavender
        [203, 166, 247], // mauve
        [245, 194, 231], // pink
        [242, 205, 205], // flamingo
        [245, 224, 220]  // rosewater
    );

    let start = std::time::Instant::now();
    let mut img = image::ImageReader::open("./ap251227-html-apollo-17.jpg")
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    dbg!(start.elapsed());

    let start = std::time::Instant::now();
    map_image_to_palette::<palette_mapper::distance::EuclidianDistance>(&mut img, &palette);
    dbg!(start.elapsed());

    let start = std::time::Instant::now();
    img.save("output.png").unwrap();
    dbg!(start.elapsed());
}
