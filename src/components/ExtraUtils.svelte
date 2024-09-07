<script lang="ts">
  import Icon from "./Icon.svelte";

  export let handle: HTMLButtonElement;
  export let loading: boolean = false;
  export let click: () => void;
</script>

<ul class="menu bg-base-200 w-56 rounded-box">
  <li class="menu-title">Utilities</li>
  <li>
    <button on:click={click} bind:this={handle}>
      {#if loading}
        <span class="loading loading-spinner loading-sm"></span>
      {:else}
        <Icon name="archive" _class="size-5" />
      {/if}
      Start Archiving
    </button>
  </li>
</ul>
