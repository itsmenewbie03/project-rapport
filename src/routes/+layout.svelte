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
      <p>
        Copyright © 2024 - Developed with 🖤 by <a
          href="https://itsmenewbie03.is-a.dev/"
          target="_blank"
          class="link link-success">@itsmenewbie03</a
        >
      </p>
    </aside>
  </footer>
</div>
