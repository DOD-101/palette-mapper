import init, {
	algorithms,
	map_image,
	Palette,
} from "./pkg/palette_mapper_web.js";

await init();

const algorithms_select = document.getElementById("algorithms"),
	img_upload = document.getElementById("img_upload"),
	img_preview = document.getElementById("img_preview");

for (const algorithm of algorithms()) {
	const elm = document.createElement("option");
	elm.textContent = algorithm;
	elm.setAttribute("value", algorithm);

	algorithms_select.appendChild(elm);
}

img_upload.addEventListener("change", (e) => {
	let files = img_upload.files;

	console.log(files[0]);

	const file_reader = new FileReader();

	file_reader.readAsArrayBuffer(files[0]);

	file_reader.onload = (ev) => {
		const bytes = ev.target.result;
		console.log(bytes);
		console.log(algorithms_select.value);

		const raw_bytes = new Uint8Array(bytes);

		let pal = [
			[255, 255, 255, 255],
			[0, 0, 0, 0],
		];

		const converted = map_image(raw_bytes, pal, algorithms_select.value);
		console.log(converted);

		// const base64 = converted.toBase64({ alphabet: "base64url" });

		const blob = new Blob([converted], { type: "image/png" });
		const imgUrl = URL.createObjectURL(blob);

		// console.log(base64);

		// img_preview.src = `data:image/png;base64,${base64}`;
		img_preview.src = imgUrl;
	};
});
