<script lang="ts">
    let { signup, message }: { signup: boolean; message?: string } = $props();
    let heading = $derived(signup ? 'Create an account' : 'Sign in');
</script>

<svelte:head><title>{heading} | Travaasa</title></svelte:head>

<section class="narrow">
    <h1>{heading}</h1>
    {#if message}<p class="notice" role="alert">{message}</p>{/if}
    <form class="stack" method="post">
        <label>Username <input name="username" autocomplete="username" maxlength={signup ? 32 : 512} required /></label>
        {#if signup}
            <label>Email <input name="email" type="email" autocomplete="email" maxlength="254" required /></label>
        {/if}
        <label>
            Password
            <input name="password" type="password" autocomplete={signup ? 'new-password' : 'current-password'}
                minlength={signup ? 12 : undefined} maxlength={signup ? 256 : 4096} required />
        </label>
        <button class="button primary">{heading}</button>
    </form>
</section>
