<script lang="ts">
    import { onMount } from 'svelte';
    import { comparisonIds, comparisonStorageKey, comparisonUrl } from '#lib/compare.js';
    import { nightlyPrice, reviewScore } from '#lib/stays.js';
    import Icon from '#lib/Icon.svelte';
    import ShareButton from '#lib/ShareButton.svelte';
    import StayImage from '#lib/StayImage.svelte';
    import type { PageProps } from './$types';

    let { data }: PageProps = $props();
    let nights = $state<number | undefined>(2);
    let tax = $state(false);
    let previous = $state<string[]>([]);
    let count = $derived(Math.min(30, Math.max(1, Math.floor(nights ?? 1))));

    onMount(() => {
        try { previous = comparisonIds(JSON.parse(localStorage.getItem(comparisonStorageKey) ?? '[]')); }
        catch { previous = []; }
    });
</script>

<svelte:head><title>Compare stays | Travaasa</title></svelte:head>

<section class="comparison">
    <a class="back-link" href="/listings">Explore stays</a>
    <div class="page-heading">
        <div>
            <h1>Compare stays</h1>
            <p class="muted">See what each stay offers and estimate the cost of your trip.</p>
        </div>
        {#if data.stays.length}<ShareButton />{/if}
    </div>
    {#if data.unavailable}
        <p class="notice">{data.unavailable === 1 ? 'One selected stay is no longer available.' : `${data.unavailable} selected stays are no longer available.`}</p>
    {/if}
    {#if data.stays.length}
        <div class="estimate-controls">
            <label>Nights <input type="number" min="1" max="30" step="1" bind:value={nights} onblur={() => { nights = count; }} /></label>
            <label class="check"><input type="checkbox" bind:checked={tax} /> Include 18% tax</label>
        </div>
        <div class="comparison-grid" data-count={data.stays.length}>
            {#each data.stays as stay (stay.id)}
                {@const rating = reviewScore(stay.reviews)}
                <article class="compare-item">
                    <StayImage source={stay.image} loading="eager" />
                    <h2><a href={`/listings/${stay.id}`}>{stay.title}</a></h2>
                    <dl>
                        <div><dt>Location</dt><dd>{stay.location}, {stay.country}</dd></div>
                        <div><dt>Stay type</dt><dd>{stay.category ?? 'Uncategorized'}</dd></div>
                        <div><dt>Guest rating</dt><dd>{rating ? `${rating}/5 · ${stay.reviews.length} ${stay.reviews.length === 1 ? 'review' : 'reviews'}` : 'No reviews yet'}</dd></div>
                        <div><dt>Per night</dt><dd>{nightlyPrice(stay.price * (tax ? 1.18 : 1))}</dd></div>
                        <div class="estimate"><dt>Estimate for {count} {count === 1 ? 'night' : 'nights'}</dt><dd>{nightlyPrice(stay.price * count * (tax ? 1.18 : 1))}</dd></div>
                    </dl>
                    <a class="button" href={`/listings/${stay.id}`}>View stay <Icon name="arrow" size={16} /></a>
                </article>
            {/each}
        </div>
    {:else}
        <div class="empty">
            <h2>Pick up to three stays</h2>
            <p>Use Compare on any listing card to add it to your shortlist.</p>
            {#if previous.length && !data.unavailable}
                <a class="button primary" href={comparisonUrl(previous)}>Open your comparison <Icon name="arrow" size={16} /></a>
            {:else}
                <a class="button primary" href="/listings#stays">Explore stays <Icon name="arrow" size={16} /></a>
            {/if}
        </div>
    {/if}
</section>
