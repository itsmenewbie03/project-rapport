<script lang="ts">
  export let title: string;
  export let description: string;
  export let hasTitle: boolean = false;
</script>

<div class="dropdown dropdown-hover dropdown-start">
  <div
    tabindex="0"
    role="button"
    class="btn btn-circle btn-ghost btn-xs text-info"
  >
    <svg
      tabindex="-1"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      class="size-4 stroke-current"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      ></path>
    </svg>
  </div>
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
