<script lang="ts">
    import AccountForm from '#lib/AccountForm.svelte';
    import type { PageProps } from './$types';
    let { form }: PageProps = $props();
</script>

<AccountForm signup={false} message={form?.message} />
