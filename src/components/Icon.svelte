<script lang="ts">
  import { get_icon } from "$lib/icons";
  export let name;
  export let _class: string | undefined = undefined;
  export let viewBox: string | undefined = undefined;
</script>

{@html get_icon(name, _class, viewBox)}
