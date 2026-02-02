import * as wasm from "../../wasm/pkg/palette_mapper_web.js";

self.onmessage = async (e) => {
  await wasm.default();

  console.log(e.data);

  self.postMessage(
    wasm.map_image(e.data.image_bytes, e.data.palette, e.data.algorithm),
  );
};
