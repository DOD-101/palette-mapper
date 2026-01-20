import init, { algorithms, map_image } from "./pkg/palette_mapper_web.js";

await init();

function to_single_dimensension_palette(pal) {
	const result = [];
	for (const color of pal) {
		let rgba;

		switch (color.length) {
			case 4:
				rgba = color;
				break;
			case 3:
				color.push(255);
				rgba = color;
				break;
			default:
				throw new Error("Invalid palette passed. This is a bug.");
		}

		result.push(...rgba);
	}

	return result;
}

/** Read the palette used to convert the image
 *
 * @param {File} file
 * @returns {number[]}
 *
 * */
function read_palette(file) {
	return new Promise((resolve, reject) => {
		const palette_reader = new FileReader();

		palette_reader.onload = (reader_ev) => {
			const contents = reader_ev.target.result;

			try {
				const json = JSON.parse(contents);

				const pal = to_single_dimensension_palette(json);

				for (const component of pal) {
					if (component > 255) {
						throw new Error("Invalid rgb component value in palette.");
					}
				}

				resolve(pal);
			} catch (e) {
				display_err(e);
			}
		};

		palette_reader.onerror = (err) => {
			reject(err);
		};

		palette_reader.readAsText(file);
	});
}

/**
 * Read the image file to be converted
 *
 * @param {File} file
 * @returns {Promise<Uint8Array>}
 * */
function read_image(file) {
	return new Promise((resolve, reject) => {
		const image_reader = new FileReader();

		image_reader.onload = (reader_ev) => {
			const bytes = new Uint8Array(reader_ev.target.result);

			resolve(bytes);
		};

		image_reader.onerror = (err) => {
			reject(err);
		};

		image_reader.readAsArrayBuffer(file);
	});
}

// TODO: Add error display for users
function display_err(err) {
	console.log(err);
}

const algorithms_select = document.getElementById("algorithms"),
	img_preview = document.getElementById("img_preview"),
	form = document.getElementById("inputs");

for (const algorithm of algorithms()) {
	const elm = document.createElement("option");
	elm.textContent = algorithm;
	elm.setAttribute("value", algorithm);

	algorithms_select.appendChild(elm);
}

form.addEventListener("submit", (ev) => {
	ev.preventDefault();

	const start = performance.now();

	const algorithm = form.algorithm.value,
		palette = form.palette.files[0],
		image = form.image.files[0];

	Promise.all([read_image(image), read_palette(palette)])
		.then((contents) => {
			const converted = map_image(contents[0], contents[1], algorithm);

			const blob = new Blob([converted], { type: "image/png" });
			const imgUrl = URL.createObjectURL(blob);

			img_preview.src = imgUrl;

			const end = performance.now();

			console.log(`Converted image. Took: ${end - start}ms`);
		})
		.catch((error) => {
			display_err(error);
		});

	console.log(
		"Converting image with params:\n(algorithm)",
		algorithm,
		"\n(palette)",
		palette,
		"\n(image)",
		image,
	);
});
