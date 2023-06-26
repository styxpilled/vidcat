<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { convertFileSrc } from '@tauri-apps/api/tauri';

  let path = '';
  let newpath = path;
  let files: string[] = [];
  let fileUrl = '';
  let currentFile = '';
  let newFileName = '';
  let currentIndex = -1;
  let video: HTMLVideoElement;
  let renameElement: HTMLInputElement;

  const changePath = async () => {
    path = await invoke('change_path', {
      path: newpath,
    });
    await fetchFiles();
  };

  const fetchFiles = async () => {
    files = await invoke('get_files');
    selectFile(0);
  };

  const renameFile = async () => {
    await invoke('rename_file', {
      from: path + '/' + currentFile,
      to: path + '/' + newFileName,
    });
    files[files.indexOf(currentFile)] = newFileName;
    currentFile = newFileName;
    renameElement.blur();
  };

  const deleteFile = async () => {
    await invoke('delete_file', {
      path: path + '/' + currentFile,
    });
    files.splice(currentIndex, 1);
    files = files;
    selectFile(currentIndex);
  };

  const selectFile = (index: number) => {
    if (index >= files.length) index = 0;
    else if (index < 0) index = files.length - 1;
    currentIndex = index;
    const src = files[index];
    fileUrl = convertFileSrc(path + '/' + src);
    newFileName = src;
    currentFile = src;
  };

  const keyHandler = async (e: KeyboardEvent) => {
    const key = e.key.toLowerCase();
    const tagName = document.activeElement?.tagName.toLocaleLowerCase();
    if (key === 'escape') renameElement.blur();
    if (tagName === 'input' || tagName === 'textarea') return;
    switch (key) {
      case ' ':
        if (video.paused) await video.play();
        else video.pause();
        e.preventDefault();
        break;
      case 'delete':
        await deleteFile();
        break;
      case 'r':
        renameElement.focus();
        renameElement.setSelectionRange(0, 0);
        e.preventDefault();
        break;
      case 'arrowup':
        selectFile(currentIndex - 1);
        break;
      case 'arrowdown':
        selectFile(currentIndex + 1);
        break;
      case 'arrowleft':
        video.currentTime -= 5;
        break;
      case 'arrowright':
        video.currentTime += 5;
        break;
      default:
        break;
    }
  };

  onMount(async () => {
    path = await invoke('get_path');
    newpath = 'D:/down';
    await changePath();
  });
</script>

<svelte:window on:keydown={keyHandler} />

<main class="container">
  <div class="path-menu">
    <p>Current path: <span class="path">{path}</span></p>
    <form on:submit|preventDefault={changePath}>
      <input type="text" name="path" id="path" bind:value={newpath} />
    </form>
  </div>

  <div class="path-menu">
    <p>Current file: <span class="path">{currentFile}</span></p>
    <form on:submit|preventDefault={renameFile}>
      <input
        type="text"
        name="rename"
        id="rename"
        bind:this={renameElement}
        bind:value={newFileName}
      />
      <button type="submit">Rename</button>
      <!-- <button type="button" class="delete" on:click={deleteFile}>Delete</button> -->
    </form>
  </div>
  <div class="main">
    <ul>
      {#each files as file, i}
        <li>
          <button
            class:active={file === currentFile}
            on:click={() => selectFile(i)}>{file}</button
          >
        </li>
      {/each}
    </ul>
    <video bind:this={video} src={fileUrl} controls>
      <track kind="captions" />
    </video>
  </div>
</main>

<style>
  .main {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  ul {
    display: block;
    max-height: 35rem;
    overflow-y: scroll;
  }

  .path-menu {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  video {
    border-radius: 0.5rem;
    max-width: 70rem;
    max-height: 35rem;
  }

  button {
    width: max-content;
  }

  button.active {
    outline: #396cd8 solid 1px;
  }

  #rename {
    width: 25rem;
  }

  li {
    margin-bottom: 0.5rem;
    white-space: break-spaces;
  }

  .path {
    text-decoration: underline;
    color: #396cd8;
  }
</style>
