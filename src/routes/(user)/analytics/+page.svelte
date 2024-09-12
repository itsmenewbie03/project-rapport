<script lang="ts">
  // NOTE: we do actually need this to import the scripts
  import VizzuPlayer from "vizzu-story";
  import UserLayout from "$components/UserLayout.svelte";
  import LoadingBars from "$components/LoadingBars.svelte";
  import { parse_feedback_data } from "$lib/parser";
  import {
    startOfMonth,
    endOfMonth,
    format,
    startOfWeek,
    endOfWeek,
    addDays,
  } from "date-fns";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto, invalidateAll } from "$app/navigation";
  import toast from "svelte-french-toast";
  import { invoke } from "@tauri-apps/api/tauri";
  import { to_timestamp } from "$root/lib/utils";
  let loaded: boolean = false;
  let story: any;

  let start_date: string = "";
  let end_date: string = "";
  let range: string = "";
  let no_feedback_data: boolean = false;
  $: range,
    (() => {
      console.log("RANGE MONITOR: ", range);
      const today = new Date();
      if (range === "monthly") {
        // Set start_date to the first day of the current month
        start_date = format(startOfMonth(today), "yyyy-MM-dd");
        // Set end_date to the last day of the current month
        end_date = format(endOfMonth(today), "yyyy-MM-dd");
        console.log("DATES: ", { start_date, end_date });
        return;
      }
      if (range === "yearly") {
        // Set start_date to the first day of the current year
        start_date = format(today, "yyyy-01-01");
        // Set end_date to the last day of the current year
        end_date = format(today, "yyyy-12-31");
        return;
      }
      if (range === "weekly") {
        // Set start_date to the first day of the current week
        start_date = format(startOfWeek(today), "yyyy-MM-dd");
        // Set end_date to the last day of the current week
        end_date = format(endOfWeek(today), "yyyy-MM-dd");
        return;
      }
    })();

  const filter = async (event: Event) => {
    localStorage.setItem("range", range);
    window.location.reload();
  };

  onMount(async () => {
    if (!$page.data.session) {
      toast.error("Please login first.");
      await goto("/login");
    }
    range = localStorage.getItem("range") ?? "monthly";
    localStorage.removeItem("range");
    // NOTE: we have to intentionally delay for 100ms
    // to allow the range value change event to propagate
    await new Promise((resolve) => setTimeout(resolve, 100));
    try {
      // NOTE: the backend will return a JSON string
      // so we will parse the data in the client side
      // (is this a bad idea? idk xD)
      // NOTE: yes dates are crazy xD
      const start = to_timestamp(start_date);
      // NOTE: we will set that the end date ends at 23:59:59
      const end = (
        parseInt(
          to_timestamp(format(addDays(end_date, 1), "yyyy-MM-dd HH:mm:ss")),
        ) - 1
      ).toString();

      console.log("Dates: ", start, end);
      // NOTE: we will make TS shutup
      // coz we know what we're doing xD
      // @ts-ignore
      const hybrid_feedback_data: string = await invoke("get_feedbacks", {
        class: "hybrid",
        start,
        end,
      }).catch((err: any): string => {
        // WARN: is this a bad idea? idk xD
        // who cares, i'll swallow the error xD
        return "";
      });
      console.log("HYBRID FEEDBACK DATA: ", hybrid_feedback_data);
      if (!hybrid_feedback_data) {
        loaded = true;
        no_feedback_data = true;
        toast.error("No feedback data found for the filter selected.");
        setTimeout(() => {
          window.location.reload();
        }, 3000);
        return;
      }
      const parsed_data = parse_feedback_data("hybrid", hybrid_feedback_data);
      const slides = [
        {
          config: {
            y: "Quality",
            x: "Total Rating",
            title: "Traditional Feedback Data",
            geometry: "line",
            split: true,
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
        slides,
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
        <div class="w-full">
          <div class="flex flex-row gap-2 float-end mr-14">
            <select class="select select-bordered" bind:value={range}>
              <option value="weekly">Weekly</option>
              <option value="monthly" selected>Monthly</option>
              <option value="yearly">Yearly</option>
              <option value="custom">Custom</option>
            </select>
            <input
              type="date"
              class="input input-bordered w-1/2"
              readonly={range !== "custom"}
              bind:value={start_date}
            />
            <input
              type="date"
              class="input input-bordered w-1/2"
              readonly={range !== "custom"}
              bind:value={end_date}
            />
            <button class="btn btn-primary" on:click={filter}>Filter</button>
          </div>
        </div>
        {#if !no_feedback_data}
          <div class="h-full pb-8">
            {@html "<vizzu-player controller></vizzu-player>"}
          </div>
        {:else}
          <div
            class="px-14 py-4 flex flex-col items-center justify-center h-full"
          >
            <h1 class="font-bold text-2xl">
              No data to show. Page will reload in 3 seconds.
            </h1>
          </div>
        {/if}
      {:else}
        <h1>Access Denied</h1>
      {/if}
    {:else}
      <LoadingBars />
    {/if}
  </div>
</UserLayout>
