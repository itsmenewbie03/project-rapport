<script lang="ts">
  import '../app.css';
  import { Toaster } from 'svelte-french-toast';
</script>

<Toaster />
<div class="flex flex-col min-h-screen bg-base-100">
  <main class="flex-grow">
    <slot />
  </main>
</div>
