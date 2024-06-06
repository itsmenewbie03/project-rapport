<script lang="ts">
  import GuestLayout from "$components/GuestLayout.svelte";
</script>

<GuestLayout>
  <p class="text-2xl">PLEASE LOGIN</p>
  <form method="POST" action="/login">
    <button type="submit" class="btn btn-primary">
      LOGIN WITH CREDENTIALS
    </button>
  </form>
</GuestLayout>
