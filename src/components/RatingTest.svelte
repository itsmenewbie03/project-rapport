<script lang="ts">
  import type { Rating } from "$lib/smileys";
  export let label: string;
  export let rating: Rating;
  import { get_smiley } from "$lib/smileys";
</script>

<span>
  <div class="flex flex-col items-center max-w-20 max-h-20">
    <div class="h-full w-full mb-2 min-w-[59px] min-h-[59px]">
      {@html get_smiley(rating)}
    </div>
    <p class="text-center text-xs font-bold">{label}</p>
  </div>
</span>
