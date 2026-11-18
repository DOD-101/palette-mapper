<script lang="ts">
  let {
    label,
    classNames = "",
    files = $bindable(),
    title = "",
    accept = "",
  }: {
    label: string;
    classNames?: string;
    files: FileList | undefined;
    title?: string;
    accept?: string;
  } = $props();
</script>

<label
  class={classNames}
  title={files && files.length >= 1 ? files[0].name : title}
>
  {label}
  <input type="file" bind:files {accept} />
</label>

<style>
  label {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text);
    text-align: center;
    background-color: var(--surface-control);
    cursor: pointer;
    border-radius: 3px;
    padding: 0;
    margin: 0.25rem 0.5rem;
    overflow: hidden;

    transition: all 200ms ease-in-out;

    &:hover {
      background-color: var(--surface-control-hover);
      scale: 1.02;
    }

    &:focus-within {
      outline: 2px solid var(--accent);
      outline-offset: 2px;
    }
  }

  /* Keep the file input focusable for keyboard users while hiding it visually. */
  input[type="file"] {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
    border: 0;
  }
</style>
