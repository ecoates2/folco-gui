<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import FolderIcon from '@lucide/svelte/icons/folder';
	import XIcon from '@lucide/svelte/icons/x';

	interface Props {
		directories: string[];
		selectedIndex?: number | null;
		onAdd?: () => void;
		onRemove?: (index: number) => void;
		onClearAll?: () => void;
		onSelect?: (index: number) => void;
		onDropPaths?: (paths: string[]) => void;
		class?: string;
	}

	let {
		directories = $bindable([]),
		selectedIndex = $bindable(null),
		onAdd,
		onRemove,
		onClearAll,
		onSelect,
		onDropPaths,
		class: className
	}: Props = $props();

	let isDragging = $state(false);

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(event: DragEvent) {
		event.preventDefault();
		isDragging = false;
	}

	async function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		// Handle file drops from the system
		const items = event.dataTransfer?.items;
		if (!items) return;

		const paths: string[] = [];

		for (const item of items) {
			if (item.kind === 'file') {
				const file = item.getAsFile();
				// In Tauri, we can get the path from the file
				// The webkitRelativePath or using Tauri's drag-drop plugin
				if (file) {
					// For now, we'll use the file name as a placeholder
					// The actual path handling will be done via Tauri's drag-drop events
					paths.push(file.name);
				}
			}
		}

		// Note: For proper file path access from system drag/drop,
		// you'll want to use @tauri-apps/plugin-drag-drop or handle it via Tauri events
		// This is a placeholder for the drop zone visual behavior
		if (paths.length > 0 && onDropPaths) {
			onDropPaths(paths);
		}
	}

	function handleItemClick(index: number) {
		selectedIndex = index;
		onSelect?.(index);
	}

	function handleRemove() {
		if (selectedIndex !== null && selectedIndex >= 0) {
			onRemove?.(selectedIndex);
			selectedIndex = null;
		}
	}

	function handleKeyDown(event: KeyboardEvent, index: number) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleItemClick(index);
		}
	}
</script>

<div class={cn('flex flex-col gap-3', className)}>
	<div class="text-lg font-semibold text-foreground">Folder(s)</div>

	<!-- Directory List / Drop Zone -->
	<div
		role="listbox"
		tabindex="0"
		aria-label="Selected directories"
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		class={cn(
			'min-h-40 rounded-md border-2 border-dashed bg-background p-2 transition-colors',
			isDragging
				? 'border-primary bg-primary/5'
				: 'border-input hover:border-muted-foreground/50',
			directories.length === 0 && 'flex items-center justify-center'
		)}
	>
		{#if directories.length === 0}
			<div class="flex flex-col items-center gap-2 text-muted-foreground">
				<FolderIcon class="size-8 opacity-50" />
				<span class="text-sm">Drop folders here or click Add</span>
			</div>
		{:else}
			<div class="flex flex-col gap-1">
				{#each directories as dir, index}
					<div
						role="option"
						tabindex="0"
						aria-selected={selectedIndex === index}
						onclick={() => handleItemClick(index)}
						onkeydown={(e) => handleKeyDown(e, index)}
						class={cn(
							'flex cursor-pointer items-center gap-2 rounded px-2 py-1.5 text-sm transition-colors',
							selectedIndex === index
								? 'bg-primary text-primary-foreground'
								: 'hover:bg-accent'
						)}
					>
						<FolderIcon class="size-4 shrink-0" />
						<span class="truncate">{dir}</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Action Buttons -->
	<div class="flex gap-2">
		<Button variant="default" size="sm" onclick={onAdd} class="flex-1">
			Add
		</Button>
		<Button
			variant="default"
			size="sm"
			onclick={handleRemove}
			disabled={selectedIndex === null}
			class="flex-1"
		>
			Remove
		</Button>
		<Button
			variant="default"
			size="sm"
			onclick={onClearAll}
			disabled={directories.length === 0}
			class="flex-1"
		>
			Clear All
		</Button>
	</div>
</div>
