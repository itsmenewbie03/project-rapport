<script lang="ts">
  import '../app.css';
  import NavBar from '$components/NavBar.svelte';
  import { Toaster } from 'svelte-french-toast';
  import BreadCrumbs from './BreadCrumbs.svelte';
  import { page } from '$app/stores';
  const is_feedback_page = $page.url.pathname.includes('feedback');
</script>

<Toaster />
<div class="flex flex-col min-h-screen">
  <NavBar />
  {#if !is_feedback_page}
    <BreadCrumbs />
  {/if}
  <main class="flex-grow">
    <slot />
  </main>
  <footer
    class="footer footer-center p-4 bg-base-300 text-base-content fixed inset-x-0 bottom-0"
  >
    <aside>
      <p>Project Rapport - Copyright © {new Date().getFullYear()}</p>
    </aside>
  </footer>
</div>
