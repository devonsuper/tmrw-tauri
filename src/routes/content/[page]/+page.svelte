<script lang="ts"> 
    /** @type {import('./$types').PageData} */    
    export let data;	

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let videoPath: string = '';
    let video: HTMLVideoElement;

    let awaitPlayTime = 250;

    onMount(async () => {
        const person :string = await invoke("get_person");

        videoPath = `/videos/${person}/${data.page}.mp4`;
    

        let timeoutId = setTimeout(() => {video.play()}, awaitPlayTime);
    });

    function handleVideoEnded(){
        goto("/");
    }

</script>

<main></main>
<div class="player">
  <video id='myVideo' bind:this={video}
    src={videoPath}
    on:ended={handleVideoEnded}
  >
    <track kind="captions"/>
  </video>
</div>

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
</style>