 <script>
    import { fade, blur } from 'svelte/transition';

    export let data;

    let inDuration = 1500;
    let outDuration = 1500;

</script>

{#key data.pathname}

    <div
        class="wrapper"
        in:fade={{duration: inDuration, delay: outDuration }}
        out:fade={{duration: outDuration }}
    >
        <slot />
    </div>
{/key}

<!-- <style>
    :global(body) {
    background-color: #000000; /* Replace with your desired color */
  }
</style> -->