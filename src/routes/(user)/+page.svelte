<script lang="ts">
  import GuestLayout from "$components/GuestLayout.svelte";
  import { page } from "$app/stores";
</script>

<GuestLayout>
  <div>
    {#if !$page.data.session}
      <p>Not logged in!</p>
    {/if}

    {#if $page.data.session}
      <p>Logged in!</p>
      {JSON.stringify($page.data.session)}
    {/if}
  </div>
</GuestLayout>
