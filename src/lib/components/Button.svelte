<!-- src/lib/components/Button.svelte -->
<script lang="ts">
  // single prop with a fixed selection of colors
  export let color: "green" | "blue" | "orange" | "yellow" | "purple" = "blue";

  // single-color gradients (light → dark of same hue)
  $: gradient = ({
    green:  'linear-gradient(180deg, #22c55e, #16a34a)',
    blue:   'linear-gradient(180deg, #3b82f6, #2563eb)',
    orange: 'linear-gradient(180deg, #f97316, #ea580c)',
    yellow: 'linear-gradient(180deg, #eab308, #ca8a04)',
    purple: 'linear-gradient(180deg, #a855f7, #7e22ce)'
  } as const)[color];
</script>

<button class="btn" on:click style={`--btn-bg:${gradient}`}>
  <slot />
</button>

<style>
  .btn {
    /* single fluid text size relative to viewport */
    font-size: clamp(0.95rem, 0.8rem + 0.6vw, 1.15rem);

    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.6em 1em;

    font-weight: 600;
    line-height: 1;
    white-space: nowrap;
    border: 0;
    border-radius: 0.5rem;

    background: var(--btn-bg);
    color: #fff;
    cursor: pointer;
    transition: filter .15s ease;
  }

  .btn:hover { filter: brightness(1.05); }
  .btn:focus-visible { outline: 2px solid rgba(255,255,255,.6); outline-offset: 2px; }
</style>