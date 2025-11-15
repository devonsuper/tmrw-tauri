<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/Button.svelte";

  let mundoBio = "In the Philippines, classrooms don’t stop at the school gate—they spill into rice fields, mangroves, and village gardens. Children learn from elders who speak of rivers as living beings, and from teachers who bring science to life through stories and experiments. Here, every young person is a seed of change, free to invent, to play, and to care for the Earth as part of their everyday learning. Let’s meet Mundo – a young 6 year old innovator. While Mundo can initially be perceived as a shy child, once you break the ice, you’ll learn that he is quite hilarious. His lively imagination and quirky ways mean you will always find him tinkering with objects around him and finding creative ways to reuse things others might throw away. Growing up with a strong connection to nature, he and his friends are currently on a ‘super-duper-ultra-important mission’ to complete their collection of leaves.  How long this mission lasts remains to be seen, though the previous one, where they collected beetles, lasted a full five hours.";
  let bakytBio = "In Kazakhstan’s classrooms, you’ll see girls leading debates, children with disabilities moving freely through safe streets, and teachers guiding with patience. Here, education isn’t a privilege—it’s a promise kept. It’s a society where inclusion is not charity but dignity, and every child learns that their place in the world is secure. Bakyt is kind and steady, with a quiet kind of bravery. Living with a visual impairment, she notices things others don’t and cares deeply about the people around her. She loves spending time with her little sister and friends—playing games, taking walks, and making music. Bakyt doesn’t need to be the loudest in the room; her strength comes from never giving up, and she shows everyone that a disability doesn’t stop her from dreaming big or living with joy. Once you speak to her, you’ll know what I mean.";
  let djamilaBio = "Step into Algeria in 2045 and you’ll see youth everywhere—running parliaments,  leading start-ups, organizing peace festivals, and sending ideas rippling across the African continent and the MENA region. It’s a society run on the spark of young people’s imagination, where joy fuels politics and classrooms overflow with movement and debate. The future here? Written in the handwriting of the young. If you don’t believe me, see for yourself! See that girl with the mic? That’s Djamila. Strong-willed and focused, Djamila always has a plan. Having grown up in a community that nurtures ambition, she dreams of becoming a surgeon and advocating for innovations in healthcare systems. Though calm and rational, her determination inspires peers who see her as a role model for what youth leadership can look like. Djamila! Take a breather, someone’s here for your autograph!";
  let suyayBio = "In Ecuador, schools sound like laughter and feel like family. Children gather in circles to share their stories, to learn how to listen, and to make peace. Art, play, and care weave together in classrooms where wellbeing is as important as math or science. Here, communities choose love over fear, and cooperation over conflict. You can see the transition from a history of conflict to a present of peace in the children here like.. Hmm.. Ah Suyay for example! Suyay is an empathetic and joyful girl. Her natural warmth makes people feel at ease and her creative spark shines in school projects and community arts. She balances her curiosity about science and technology with a strong grounding in ancestral wisdom passed down from her grandmother, which can make her seem wise beyond her years. For her, education isn’t just about grades but about peace, care and collective well-being – she can speak to this much better than I can!";
  let kwabenaBio = "Ghana is buzzing with youth energy—you can see young innovators coding, gaming, debating, and deciding how AI serves their future. Here, young people make sure that technology bends toward humanity. It’s not about overreliance—it’s about balance. The rhythm of Ghana in 2045? Youth voices loud, proud, and in charge. There’s Kwabena! Kwabena rarely takes things at face value – making him an excellent debater and a creative programmer. His confidence and leadership skills can be seen on display when he represents Ghana as a UNICEF AI for Education Global Ambassador, or, on a more personal level - when planning an ambitious, spontaneous hike with his friends. Hey Chale, what's up?! Someone’s here to meet you!";

  let person: string | null = null;
  let bioText = "";

  let buttonColor: string = "blue";
  const personToColor: Map<string, string> = new Map();
  personToColor.set("mundo", "green");
  personToColor.set("kwabena", "blue");
  personToColor.set("bakyt", "orange");
  personToColor.set("suyay", "yellow");
  personToColor.set("djamila", "purple");

  const personToBio: Map<string, string> = new Map();
  personToBio.set("mundo", mundoBio);
  personToBio.set("kwabena", kwabenaBio);
  personToBio.set("bakyt", bakytBio);
  personToBio.set("suyay", suyayBio);
  personToBio.set("djamila", djamilaBio);

  onMount(async () => {
    person = await invoke<string>("get_person");
    buttonColor = personToColor.get(person) ?? "blue";
    bioText = personToBio.get(person) ?? "";
  });
</script>

<main>
  {#if person && bioText}
    <section class="bio">
      <pre class="bio-text">{bioText}</pre>
    </section>
  {:else}
    <section class="bio loading">
      Loading…
    </section>
  {/if}

  <div class="buttons">
    <Button on:click={() => goto("/")} color={buttonColor}>Back</Button>
  </div>
</main>

<style>
  main {
    min-height: 100vh;
    display: flex;
    flex-direction: column;

    /* grey/green background */
    background: linear-gradient(135deg, #111827, #374151, #065f46);
    color: #ecfdf5;
  }

  .bio {
    flex: 1;
    padding: 4rem 12vw;
    box-sizing: border-box;
    overflow: auto;
  }

  .bio-text {
    margin: 0;
    max-width: 70ch;
    margin-inline: auto;

    font-family: "IBM Plex Mono", ui-monospace, SFMono-Regular, Menlo, Monaco,
      Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 1.6rem;       /* large */
    line-height: 2;
    letter-spacing: 0.03em;
    white-space: pre-wrap;   /* wrap plaintext lines */

    text-shadow: 0 0 12px rgba(0, 0, 0, 0.45);
  }

  .bio.loading {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.4rem;
  }

  .buttons {
    padding: 1.5rem 12vw 2.5rem;
    display: flex;
    justify-content: flex-end;
  }
</style>