<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import * as EmojiPicker from '$lib/components/ui/emoji-picker';
  import * as Resizable from "$lib/components/ui/resizable/index.js";

  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<Resizable.PaneGroup direction="horizontal" class="max-w-md rounded-lg border">
  <Resizable.Pane defaultSize={50}>
    <div class="flex h-50 items-center justify-center p-6">
      <span class="font-semibold">One</span>
    </div>
  </Resizable.Pane>
  <Resizable.Handle />
  <Resizable.Pane defaultSize={50}>
    <Resizable.PaneGroup direction="vertical">
      <Resizable.Pane defaultSize={25}>
        <div class="flex h-full items-center justify-center p-6">
          <span class="font-semibold">Two</span>
        </div>
      </Resizable.Pane>
      <Resizable.Handle />
      <Resizable.Pane defaultSize={75}>
        <div class="flex h-full items-center justify-center p-6">
          <span class="font-semibold">Three</span>
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  </Resizable.Pane>
</Resizable.PaneGroup>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <Button>Click me</Button>

<!-- <EmojiPicker.Root onSelect={(selected) => console.log('Selected emoji:', selected)}>
	<EmojiPicker.Viewport>
		<EmojiPicker.Search />
		<EmojiPicker.List />
	</EmojiPicker.Viewport>
</EmojiPicker.Root> -->

  <!-- <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p> -->

  <!-- <h1 class="text-3xl font-bold underline">
  Hello world!
  </h1> -->

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>
