interface ImgData {
  data: Blob;
  file_name: string;
}

const converted: ImgData = $state({
  data: new Blob(),
  file_name: "",
});

const original: ImgData = $state({
  data: new Blob(),
  file_name: "",
});

export const img_data = {
  original: original,
  converted: converted,
};
