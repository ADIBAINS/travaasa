<script lang="ts">
    import StayCatalog from '#lib/StayCatalog.svelte';
    import type { PageProps } from './$types';

    let { data, form }: PageProps = $props();
</script>

<StayCatalog {...data} actionMessage={form?.message} savedChange={form?.savedChange} signedIn={Boolean(data.user)} />
