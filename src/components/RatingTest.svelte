<script lang="ts">
  import type { Rating } from "$lib/smileys";
  export let label: string;
  export let rating: Rating;
  export let group: number;
  export let value: number;
  import { get_smiley } from "$lib/smileys";
</script>

<span>
  <input type="radio" name="rating-1" {value} bind:group hidden />
  <!-- NOTE: we don't care about keyboard events -->
  <!-- NOTE: we will deploy this in a touchscreen device -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class={`flex flex-col items-center max-w-20 max-h-20 ${group === value ? "bg-red-200" : ""}`}
    on:click={() => (group = value)}
  >
    <div class="h-full w-full mb-2 min-w-[59px] min-h-[59px]">
      {@html get_smiley(rating)}
    </div>
    <p class="text-center text-xs font-bold">{label}</p>
  </div>
</span>
