let data: Blob | undefined = $state();

export const converted_img = {
  get data() {
    return data;
  },
  set data(blob) {
    data = blob;
  },
};
