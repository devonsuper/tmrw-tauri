 <script>
    import { slide, blur } from 'svelte/transition';

    export let data;

    let inDuration = 500;
    let outDuration = 500;



</script>

{#key data.pathname}

    <div
        class="wrapper"
        in:blur={{duration: inDuration, delay: outDuration }}
        out:blur={{duration: outDuration }}
    >
        <slot />
    </div>
{/key}

<!-- <style>
    :global(body) {
    background-color: #000000; /* Replace with your desired color */
  }
</style> -->