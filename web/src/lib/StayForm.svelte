<script lang="ts">
    import { categories, type Detail } from './stays.js';

    let { stay, fields, message }: { stay?: Detail; fields?: Record<string, string>; message?: string } = $props();
    let heading = $derived(stay ? 'Edit your stay' : 'Host a stay');
</script>

<svelte:head><title>{heading} | Travaasa</title></svelte:head>

<section class="narrow">
    <h1>{heading}</h1>
    {#if message}<p class="notice" role="alert">{message}</p>{/if}
    <form class="stack" method="post" enctype="multipart/form-data">
        <label>Title <input name="title" maxlength="150" required value={fields?.title ?? stay?.title ?? ''} /></label>
        <label>Description <textarea name="description" rows="6" maxlength="5000" required value={fields?.description ?? stay?.description ?? ''}></textarea></label>
        <div class="field-row">
            <label>Location <input name="location" maxlength="150" required value={fields?.location ?? stay?.location ?? ''} /></label>
            <label>Country <input name="country" maxlength="100" required value={fields?.country ?? stay?.country ?? ''} /></label>
        </div>
        <div class="field-row">
            <label>Price per night (₹) <input name="price" type="number" min="0" max="1000000000" step="any" required value={fields?.price ?? stay?.price ?? ''} /></label>
            <label>
                Category
                <select name="category" value={fields?.category ?? stay?.category ?? ''}>
                    <option value="">Uncategorized</option>
                    {#each categories as category}<option value={category}>{category}</option>{/each}
                </select>
            </label>
        </div>
        <label>
            {stay?.image ? 'Replace photo' : 'Photo (optional)'}
            <input type="file" name="image" accept="image/jpeg,image/png" />
            <small class="muted">JPEG or PNG up to 5 MB</small>
        </label>
        <button class="button primary">{stay ? 'Save changes' : 'Publish stay'}</button>
    </form>
</section>
