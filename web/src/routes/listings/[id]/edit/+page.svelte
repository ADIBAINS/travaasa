<script lang="ts">
    import StayForm from '#lib/StayForm.svelte';
    import type { PageProps } from './$types';
    let { data, form }: PageProps = $props();
</script>

<StayForm stay={data.stay} fields={form?.fields} message={form?.message} />
