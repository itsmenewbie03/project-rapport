<script lang="ts">
  export let title: string;
  export let description: string;
  export let hasTitle: boolean = false;
</script>

<div class="dropdown dropdown-hover dropdown-start">
  <div class="badge badge-primary badge-outline ml-2">Quick Tip</div>
  <div
    tabindex="-1"
    class="card compact dropdown-content z-[1] shadow bg-primary-content rounded-box w-64"
  >
    <div tabindex="-1" class="card-body">
      {#if hasTitle}
        <h2 class="card-title">{title}</h2>
      {/if}
      <p>{description}</p>
    </div>
  </div>
</div>
