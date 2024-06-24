<script lang="ts">
  import { page } from "$app/stores";
  import Icon from "./Icon.svelte";
  let paths = $page.url.pathname.split("/");
  paths.shift();
  paths = paths.map((value) => {
    return value.charAt(0).toUpperCase() + value.slice(1);
  });
</script>

<div class="text-lg breadcrumbs ml-14">
  <ul>
    {#each paths as path}
      <li>
        <span class="inline-flex gap-2 items-center">
          <Icon name={path.toLowerCase()} />
          {path}
        </span>
      </li>
    {/each}
  </ul>
</div>
