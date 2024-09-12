<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import toast from "svelte-french-toast";
  import { startOfMonth, endOfMonth, format } from "date-fns";
  import { startOfWeek } from "date-fns";
  import { endOfWeek } from "date-fns";
  import { compareAsc } from "date-fns";
  import { addDays } from "date-fns";
  import { to_timestamp } from "$root/lib/utils";
  let end_date_input: HTMLInputElement;
  let start_date: string;
  let generating: boolean = false;

  $: start_date,
    (() => {
      if (start_date) {
        end_date_input.min = start_date;
      }
    })();

  let end_date: string;
  $: end_date,
    (() => {
      if (end_date) {
        const cmp = compareAsc(start_date, end_date);
        if (cmp > 0) {
          toast.error("End Date cannot be before the Start Date");
          end_date = format(addDays(start_date, 1), "yyyy-MM-dd");
        }
      }
    })();

  let range: string;
  // NOTE: yes, this is crazy xD
  $: range,
    (() => {
      const today = new Date();
      if (range === "monthly") {
        // Set start_date to the first day of the current month
        start_date = format(startOfMonth(today), "yyyy-MM-dd");
        // Set end_date to the last day of the current month
        end_date = format(endOfMonth(today), "yyyy-MM-dd");
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

  const generate = async () => {
    // NOTE: yes dates are crazy xD
    const start = to_timestamp(start_date);
    // NOTE: we will set that the end date ends at 23:59:59
    const end =
      to_timestamp(format(addDays(end_date, 1), "yyyy-MM-dd HH:mm:ss")) - 1;
    generating = true;
    try {
      const res: string = await invoke("generate_report", {
        start,
        end,
      });
      toast.success(res);
    } catch (err: any) {
      // NOTE: just delaying the function to show the loading spinner
      // for a second when an error occurs as it errors out too fast xD
      await new Promise((resolve) => setTimeout(resolve, 1000));
      toast.error(err);
    } finally {
      generating = false;
    }
  };
</script>

<div class="card bg-base-200 w-full max-w-md shadow-xl bordered mt-6">
  <div class="card-body">
    <h2 class="card-title">Report Generator</h2>
    <label class="form-control w-full">
      <div class="label">
        <span class="label-text">Select Range</span>
      </div>
      <select class="select select-bordered w-full" bind:value={range}>
        <option value="weekly">Weekly</option>
        <option value="monthly" selected>Monthly</option>
        <option value="yearly">Yearly</option>
        <option value="custom">Custom</option>
      </select>
    </label>
    <div class="flex flex-row gap-1">
      <label class="form-control w-full">
        <div class="label">
          <span class="label-text">Start Date</span>
        </div>
        <input
          type="date"
          class="input input-bordered w-full"
          readonly={range !== "custom"}
          bind:value={start_date}
        />
      </label>
      <label class="form-control w-full">
        <div class="label">
          <span class="label-text">End Date</span>
        </div>
        <input
          type="date"
          class="input input-bordered w-full"
          readonly={range !== "custom"}
          bind:value={end_date}
          bind:this={end_date_input}
        />
      </label>
    </div>
    <div class="card-actions justify-end mt-2">
      <button class="btn btn-primary" on:click={generate} disabled={generating}>
        Generate
        {#if generating}
          <span class="loading loading-spinner loading-xs"></span>
        {/if}
      </button>
    </div>
  </div>
</div>
