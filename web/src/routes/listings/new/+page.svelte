<script lang="ts">
    import StayForm from '#lib/StayForm.svelte';
    import type { PageProps } from './$types';
    let { form }: PageProps = $props();
</script>

<StayForm fields={form?.fields} message={form?.message} />
