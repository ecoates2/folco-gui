<script lang="ts">
  import { DirectoryPicker } from '$lib/components/app/directory-picker';

  // For now, we'll use a mock for the dialog
  // Once you install @tauri-apps/plugin-dialog, uncomment the import below
  // import { open } from '@tauri-apps/plugin-dialog';

  let directories = $state<string[]>([]);
  let selectedIndex = $state<number | null>(null);

  async function handleAdd() {
    // Mock implementation - replace with actual Tauri dialog once plugin is installed
    // const selected = await open({
    //   directory: true,
    //   multiple: true,
    //   title: 'Select Folder(s)'
    // });
    //
    // if (selected) {
    //   const paths = Array.isArray(selected) ? selected : [selected];
    //   directories = [...directories, ...paths.filter(p => !directories.includes(p))];
    // }

    // Placeholder for testing without the plugin
    const mockPath = `/home/user/folder-${directories.length + 1}`;
    if (!directories.includes(mockPath)) {
      directories = [...directories, mockPath];
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
