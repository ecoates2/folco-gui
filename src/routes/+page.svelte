<script lang="ts">
  import { onMount } from 'svelte';
  import { renderer } from '$lib/stores/renderer.svelte';

  import { ColorPicker } from '$lib/components/app/color-picker';
  import { CustomizationOption } from '$lib/components/app/customization-option';
  import { DirectoryPicker } from '$lib/components/app/directory-picker';
  import { IconPreview } from '$lib/components/app/icon-preview';

  import 'iconify-picker';

  import { open } from '@tauri-apps/plugin-dialog';

  import * as EmojiPicker from '$lib/components/ui/emoji-picker';
  import type { SelectedEmoji, EmojiPickerSkin } from '$lib/components/ui/emoji-picker/types';

  onMount(() => renderer.init());

  let colorEnabled = $state(false);
  let emojiEnabled = $state(false);
  let iconEnabled = $state(false);

  /** Last selected emoji — used to re-emit the overlay when the skin tone changes. */
  let lastEmoji = $state<SelectedEmoji | null>(null);

  function handleEmojiSelect(emoji: SelectedEmoji) {
    lastEmoji = emoji;
    renderer.setOverlayEmoji(emoji.emoji, 'bottom-right', 0.5);
  }

  function handleSkinChange(skin: EmojiPickerSkin) {
    if (!lastEmoji || lastEmoji.data.skins.length <= 1) return;
    const native = lastEmoji.data.skins[skin].native;
    lastEmoji = { ...lastEmoji, emoji: native, skin };
    renderer.setOverlayEmoji(native, 'bottom-right', 0.5);
  }

  let directories = $state<string[]>([]);
  let selectedIndex = $state<number | null>(null);

  async function handleAdd() {
    const selected = await open({
      directory: true,
      multiple: true,
      title: 'Select Folder(s)'
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      directories = [...directories, ...paths.filter(p => !directories.includes(p))];
    }
  }

  function handleRemove(index: number) {
    directories = directories.filter((_, i) => i !== index);
    selectedIndex = null;
  }

  function handleClearAll() {
    directories = [];
    selectedIndex = null;
  }

  function handleDropPaths(paths: string[]) {
    // Filter out duplicates
    const newPaths = paths.filter(p => !directories.includes(p));
    directories = [...directories, ...newPaths];
  }
</script>

<main class="container mx-auto max-w-2xl p-6">
  <h1 class="mb-6 text-2xl font-bold text-foreground">Folder Customization</h1>

  <div class="mb-6 flex flex-col gap-3">
    <CustomizationOption
      label="Color"
      bind:enabled={colorEnabled}
      onToggle={(on) => renderer.setFolderColorTargetEnabled(on)}
    >
      <ColorPicker
        onchange={(color) => renderer.setFolderColorTarget(color.r, color.g, color.b)}
      />
    </CustomizationOption>

    <CustomizationOption
      label="Emoji Overlay"
      bind:enabled={emojiEnabled}
      onToggle={(on) => renderer.setOverlayEnabled(on)}
    >
      <EmojiPicker.Root onSelect={handleEmojiSelect} onSkinChange={handleSkinChange}>
        <EmojiPicker.Viewport>
          <EmojiPicker.Search />
          <EmojiPicker.List />
          <EmojiPicker.Footer>
            <EmojiPicker.SkinToneSelector />
          </EmojiPicker.Footer>
        </EmojiPicker.Viewport>
      </EmojiPicker.Root>
    </CustomizationOption>

    <CustomizationOption
      label="Icon"
      bind:enabled={iconEnabled}
      onToggle={(on) => renderer.setDecalEnabled(on)}
    >
      <iconify-picker
        collection="mdi"
        hide-search
        hide-collection
        page-size="30">
      </iconify-picker>
    </CustomizationOption>
  </div>

  <IconPreview class="mb-6" />

  <DirectoryPicker
    bind:directories
    bind:selectedIndex
    onAdd={handleAdd}
    onRemove={handleRemove}
    onClearAll={handleClearAll}
    onDropPaths={handleDropPaths}
    class="max-w-md"
  />

  <!-- Debug output -->
  <div class="mt-6 rounded-md border border-border bg-muted/50 p-4">
    <p class="text-sm text-muted-foreground">
      Selected directories: {directories.length}
    </p>
    <p class="text-sm text-muted-foreground">
      Selected index: {selectedIndex ?? 'none'}
    </p>
  </div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>
