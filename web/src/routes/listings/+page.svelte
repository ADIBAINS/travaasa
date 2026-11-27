<script lang="ts">
    import StayCatalog from '#lib/StayCatalog.svelte';
    import type { PageProps } from './$types';

    let { data }: PageProps = $props();
</script>

<StayCatalog {...data} />
