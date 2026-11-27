<script lang="ts">
    import Icon from './Icon.svelte';

    let { source, alt = '', loading = 'lazy' }: {
        source: string | null;
        alt?: string;
        loading?: 'lazy' | 'eager';
    } = $props();

    let failedSource = $state<string | null>(null);
</script>

{#if source && source !== failedSource}
    <img src={source} {alt} {loading} onerror={() => { failedSource = source; }} />
{:else}
    <div class="no-image"><Icon name="photo" size={26} /><span>No photo</span></div>
{/if}
