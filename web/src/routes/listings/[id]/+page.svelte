<script lang="ts">
    import { nightlyPrice } from '#lib/stays.js';
    import type { PageProps } from './$types';
    let { data, form }: PageProps = $props();
    let stay = $derived(data.stay);
    let ownStay = $derived(data.user && data.user.id === stay.owner?.id);
    let locationQuery = $derived(encodeURIComponent(`${stay.location}, ${stay.country}`));
</script>

<svelte:head><title>{stay.title} | Travaasa</title></svelte:head>

<a class="back-link" href="/listings">Explore stays</a>
{#if form?.message}<p class="notice" role="alert">{form.message}</p>{/if}
<article class="detail">
    <div class="detail-heading">
        <div>
            <h1>{stay.title}</h1>
            <p class="muted">{stay.location}, {stay.country}{stay.category ? ` · ${stay.category}` : ''}</p>
        </div>
        {#if data.user}
            <form method="post" action={data.saved ? '?/unsave' : '?/save'}>
                <button class="button" aria-pressed={data.saved}>{data.saved ? 'Remove from saved' : 'Save stay'}</button>
            </form>
        {:else}
            <a class="button" href={`/login?next=/listings/${stay.id}`}>Sign in to save</a>
        {/if}
    </div>
    {#if stay.image}
        <img class="hero-image" src={stay.image} alt={stay.title} />
    {/if}
    <section class="detail-section">
        <h2>{stay.owner ? `Hosted by ${stay.owner.username}` : 'About this stay'}</h2>
        <p><strong>{nightlyPrice(stay.price)}</strong> / night</p>
        <p class="description">{stay.description}</p>
        {#if ownStay}
            <div class="owner-tools">
                <a class="button" href={`/listings/${stay.id}/edit`}>Edit stay</a>
                <details>
                    <summary>Delete stay</summary>
                    <form method="post" action="?/deleteStay" class="stack">
                        <label class="check"><input type="checkbox" name="confirm" value="yes" required /> Delete this stay and its reviews</label>
                        <button class="button danger">Confirm deletion</button>
                    </form>
                </details>
            </div>
        {/if}
    </section>
    <section class="detail-section">
        <h2>Where you’ll be</h2>
        {#if data.mapImage}
            <img class="map-image" src={data.mapImage} alt={`Map near ${stay.location}`} loading="lazy" />
        {/if}
        <a href={`https://www.openstreetmap.org/search?query=${locationQuery}`}>Open map for {stay.location}</a>
    </section>
    <section class="detail-section">
        <h2>Reviews {stay.reviews.length ? `(${stay.reviews.length})` : ''}</h2>
        {#if stay.reviews.length === 0}<p class="muted">No reviews yet.</p>{/if}
        <div class="reviews">
            {#each stay.reviews as review (review.id)}
                <article class="review">
                    <p><strong>{review.author?.username ?? 'Former member'}</strong> · {review.rating}/5</p>
                    <p class="description">{review.comment}</p>
                    {#if data.user && data.user.id === review.author?.id}
                        <form method="post" action="?/deleteReview">
                            <input type="hidden" name="reviewId" value={review.id} />
                            <button class="text-button danger-text">Delete review</button>
                        </form>
                    {/if}
                </article>
            {/each}
        </div>
        {#if data.user}
            <form class="stack review-form" method="post" action="?/review">
                <h3>Leave a review</h3>
                <label>
                    Rating
                    <select name="rating" required>
                        <option value="">Select a rating</option>
                        {#each [5, 4, 3, 2, 1] as stars}<option value={stars}>{stars} out of 5</option>{/each}
                    </select>
                </label>
                <label>Comment <textarea name="comment" rows="4" maxlength="2500" required></textarea></label>
                <button class="button primary">Post review</button>
            </form>
        {/if}
    </section>
</article>
