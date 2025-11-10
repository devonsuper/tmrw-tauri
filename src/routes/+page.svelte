<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { preloadCode, preloadData } from '$app/navigation';

  import { fade } from 'svelte/transition';

  import Button from '$lib/components/Button.svelte';

  let videoPath: string = '';
  let selectionMode = false;

  const timeOutLength = 30000; // 30 seconds
  let timeoutId: ReturnType<typeof setTimeout>;

  let buttonColor: string = "blue";
  const personToColor: Map<string, string> = new Map();
  personToColor.set("mundo", "green");
  personToColor.set("kwabena", "blue");
  personToColor.set("bukyt", "orange");
  personToColor.set("suyay", "yellow");
  personToColor.set("djamila", "purple");


  function handleVideoClick(e: MouseEvent){
      selectionMode = true;

      clearTimeout(timeoutId)
      timeoutId = setTimeout(() => {selectionMode = false}, timeOutLength);
  }

  onMount(async () => {
    const person :string = await invoke("get_person");

    videoPath = `/videos/${person}/idle.mp4`
  
    buttonColor = personToColor.get(person);
  });
</script>
  
<main>
<div class="player">
  <video id='myVideo' class:blurred={selectionMode}
    src={videoPath}
    autoplay
    loop
    onclick={handleVideoClick}
  >
    <track kind="captions" />
  </video>
</div>


{#if selectionMode}
  <div class="buttons" in:fade>
    <Button on:click={() => {goto("/content/learning")}} color={buttonColor}>What's learning like?</Button>
    <Button on:click={() => {goto("/content/challenges")}} color={buttonColor}>What new challenges are you facing?</Button>
    <Button on:click={() => {goto("/content/opportunities")}} color={buttonColor}>What new opportunities do you have? </Button>
    <Button on:click={() => {goto("/aboutme")}} color={buttonColor}>About Me</Button>
  </div>
{/if}

</main>

<style>
  .player {
    width: 100%;
    height: 100%;
    position: absolute;
    padding:0;
    margin:0;
    left: 0px;
    top: 0px;
    z-index: -1000;
    overflow:hidden;
  }

  .player video {
    display: block;
    width: 100%;
    height: 100%;  /* height: 100vh; makes it the whole height of the screen */
    object-fit: cover;  
  }

  .blurred {
    filter: blur(12px);
  }

  .buttons {
    position: fixed;     /* anchor to viewport */
    left: 0;
    right: 0;
    top: 66vh;           /* start 2/3 down the screen */
    display: flex;
    flex-direction: column;
    align-items: stretch; /* children fill width */
    gap: 1.5rem;
    margin: 0;
    padding: 0;
  }

  .buttons .btn { width: 100%; }

</style>
