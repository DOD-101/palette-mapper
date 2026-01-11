#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use image::{DynamicImage, ImageBuffer, Rgba};
use palette_mapper::{
    Palette, color_pallete,
    distance::{Algorithms, EuclideanDistance},
    map_image_to_palette,
};
use rayon::iter::ParallelIterator;
use std::{hint::black_box, sync::LazyLock, time::Duration};
use strum::VariantArray;

static TESTING_PALLETE: LazyLock<Palette> = LazyLock::new(|| {
    color_pallete!(
        [255, 0, 0, 255],     // Red
        [255, 128, 0, 255],   // Orange
        [255, 255, 0, 255],   // Yellow
        [128, 255, 0, 255],   // Yellow-Green
        [0, 255, 0, 255],     // Green
        [0, 255, 128, 255],   // Spring Green
        [0, 255, 255, 255],   // Cyan
        [0, 128, 255, 255],   // Azure
        [0, 0, 255, 255],     // Blue
        [128, 0, 255, 255],   // Violet
        [255, 0, 255, 255],   // Magenta
        [255, 0, 128, 255],   // Rose
        [255, 255, 255, 255], // White
        [192, 192, 192, 255], // Light Gray
        [64, 64, 64, 255],    // Dark Gray
        [0, 0, 0, 255]        // Black
    )
});

fn img_buf_noise(size: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = image::ImageBuffer::new(size.0, size.1);

    img.par_enumerate_pixels_mut()
        .for_each(|px| *px.2 = Rgba::<u8>::from(rand::random::<[u8; 4]>()));

    img
}

fn map_imgage_to_palette(c: &mut Criterion) {
    let sizes = [(500, 500), (1080, 720), (1920, 1080), (3440, 1440)];

    let mut group = c.benchmark_group("map_image_to_palette");
    group.throughput(criterion::Throughput::Elements(1));
    group.warm_up_time(Duration::from_secs(5));
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(10));

    for algorithm in <Algorithms as VariantArray>::VARIANTS {
        for size in sizes {
            let input = (size, algorithm);
            group.bench_with_input(
                BenchmarkId::from_parameter(format!(
                    "{}x{} palette-16 {}",
                    input.0.0, input.0.1, input.1
                )),
                &input,
                |b, &input| {
                    let mut img = DynamicImage::from(img_buf_noise(input.0));

                    let palette = &TESTING_PALLETE;

                    b.iter(|| {
                        map_image_to_palette(
                            black_box(&mut img),
                            black_box(palette),
                            &black_box(EuclideanDistance),
                        );
                    });
                },
            );
        }
    }
}

criterion_group!(benches, map_imgage_to_palette);
criterion_main!(benches);
