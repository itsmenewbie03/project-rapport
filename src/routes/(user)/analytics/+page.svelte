<script lang="ts">
  // NOTE: we do actually need this to import the scripts
  import VizzuPlayer from "vizzu-story";
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { parse_feedback_data } from "$lib/parser";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import toast from "svelte-french-toast";
  import { invoke } from "@tauri-apps/api/tauri";
  let loaded: boolean = false;
  let story: any;
  let lazy_loading: boolean = true;
  onMount(async () => {
    if (!$page.data.session) {
      toast.error("Please login first.");
      await goto("/login");
    }
    try {
      // NOTE: the backend will return a JSON string
      // so we will parse the data in the client side
      // (is this a bad idea? idk xD)
      const hybrid_feedback_data: string = await invoke("get_feedbacks", {
        class: "hybrid",
      });
      const parsed_data = parse_feedback_data("hybrid", hybrid_feedback_data);
      const slides = [
        {
          config: {
            x: "Quality",
            y: "Total Rating",
            title: "Traditional Feedback Data",
          },
        },
        {
          config: {
            x: "Access and Facilities",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Assurance",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Communication",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Integrity",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Outcome",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Overall Satisfaction",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Reliability",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Responsiveness",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
        {
          config: {
            x: "Value for Money",
            y: "Emotion",
            title: "Hybrid Feedback Data",
          },
        },
      ];
      const _story = {
        data: parsed_data,
        slides: slides,
      };
      story = _story;
    } catch (err: any) {
      toast.error(err.message);
    }
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(async () => {
      loaded = true;
      setTimeout(() => {
        const vp = document.querySelector("vizzu-player");
        // @ts-ignore
        vp.style.cssText =
          "width: 100%; height: calc(100vh-144px); padding-bottom: 1.5rem; padding-left: 3.5rem; padding-right: 3.5rem";
        // @ts-ignore
        vp.slides = story;
      }, 100);
    }, 1000);
  });
</script>

<UserLayout>
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        {@html "<vizzu-player controller></vizzu-player>"}
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
