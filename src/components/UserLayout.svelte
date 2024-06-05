<script lang="ts">
  import "../app.css";
  import NavBar from "$components/NavBar.svelte";
  import { Toaster } from "svelte-french-toast";
</script>

<Toaster />
<div class="flex flex-col min-h-screen">
  <NavBar />
  <main class="flex-grow">
    <slot />
  </main>
  <footer
    class="footer footer-center p-4 bg-base-300 text-base-content absolute inset-x-0 bottom-0"
  >
    <aside>
      <p>Project Rapport - Copyright © 2024</p>
    </aside>
  </footer>
</div>
