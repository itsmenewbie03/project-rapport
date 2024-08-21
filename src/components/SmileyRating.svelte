<script lang="ts">
  import type { Rating } from "$lib/smileys";
  export let label: string;
  export let rating: Rating;
  export let group: number;
  export let value: number;
  import { get_smiley } from "$lib/smileys";
</script>

<input type="radio" name="rating-1" {value} bind:group hidden />
<!-- NOTE: we don't care about keyboard events -->
<!-- NOTE: we will deploy this in a touchscreen device -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class={`flex flex-col items-center p-4 border border-black rounded-md min-w-[127px] max-w-[127px] ${group === value ? "bg-gray-300" : ""}`}
  on:click={() => (group = value)}
>
  <div class="h-full w-full min-w-[65px] max-w-[65px] basis-4/5 mb-2">
    {@html get_smiley(rating)}
  </div>
  <div class="basis-1/5">
    <p class="text-center text-xs font-bold">{label}</p>
  </div>
</div>
