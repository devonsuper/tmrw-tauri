<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { preloadCode, preloadData } from '$app/navigation';

  import { fade} from 'svelte/transition';

  import Button from '$lib/components/Button.svelte';

  let videoPath: string = '';
  let selectionMode = false;

  const timeOutLength = 25000; // 25 seconds
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
  
    buttonColor = personToColor.get(person) ?? 'blue';
  });
</script>
  
<main>
<div class="player">
    <div class="overlay" class:blurred={selectionMode} in:fade></div>
    <video
      id="myVideo"
      src={videoPath}
      autoplay
      loop
      muted
      playsinline
      preload
      onclick={handleVideoClick}
      >
    <track kind="captions" />
  </video>
</div>


{#if selectionMode}
  <div class="buttons" in:fade out:fade>
    <Button on:click={() => {goto("/content/learning")}} color={buttonColor}>What's learning like?</Button>
    <Button on:click={() => {goto("/content/challenges")}} color={buttonColor}>What new challenges are you facing?</Button>
    <Button on:click={() => {goto("/content/opportunities")}} color={buttonColor}>What new opportunities do you have? </Button>
    <Button on:click={() => {goto("/aboutme")}} color={buttonColor}>Tell me about yourself</Button>
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
    width: 100vh;
    height: 100%;  /* height: 100vh; makes it the whole height of the screen */
    object-fit: cover;  
  }

  .blurred {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.3);
    z-index: 1;
  }

  .buttons {
    position: fixed;        /* anchor to viewport */
    inset: 0;               /* top:0; right:0; bottom:0; left:0 */
    
    display: flex;
    flex-direction: column;
    justify-content: center;  /* vertical center */
    align-items: center;      /* horizontal center */

    gap: 6vh;
    margin: 0;
    padding: 0 10vw;        /* side padding so buttons don't touch edges */
  }

  .buttons .btn { width: 100%; }

</style>
