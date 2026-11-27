<script lang="ts">
    import { page } from '$app/state';
</script>

<section class="empty">
    <h1>{page.status === 404 ? 'Page not found' : 'Something went wrong'}</h1>
    <p>{page.error?.message}</p>
    <a href="/listings">Explore stays</a>
</section>
