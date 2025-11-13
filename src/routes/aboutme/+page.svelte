<script lang="ts"> 
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    import Button from '$lib/components/Button.svelte';

    let person: string;

    onMount(async () => {
        person = await invoke("get_person");
    });

</script>

<main>
    
    {#key person}
     <!-- <embed src="/{person}.txt"> -->
        <embed class="text" src="bundle://localhost/assets/{person}.txt" color="white" type="text/plain">
    {/key}

    <div class="buttons">
        <Button on:click={() => {goto("/")}} color=blue>Back</Button>
    </div>
</main>

<style>

  .buttons {
    position: fixed;     /* anchor to viewport */
    left: 0;
    right: 0;
    top: 66vh;           /* start 2/3 down the screen */
    display: flex;
    flex-direction: column;
    /* align-items: stretch; children fill width */
    gap: 1.5rem;
    margin: 0;
    padding: 0;
  }

  embed {
    color: white
  }

</style>