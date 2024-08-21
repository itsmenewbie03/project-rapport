<script lang="ts">
  import SmileyRating from "./SmileyRating.svelte";
  export let group: number;
</script>

<div class="flex flex-row justify-between w-full">
  <SmileyRating
    bind:group
    label="Very Dissatisfied"
    value={1}
    rating="very_dissatisfied"
  />
  <SmileyRating
    bind:group
    label="Dissatisfied"
    value={2}
    rating="dissatisfied"
  />
  <SmileyRating bind:group label="Neutral" value={3} rating="neutral" />
  <SmileyRating bind:group label="Satisfied" value={4} rating="satisfied" />
  <SmileyRating
    bind:group
    label="Very Satisfied"
    value={5}
    rating="very_satisfied"
  />
</div>
