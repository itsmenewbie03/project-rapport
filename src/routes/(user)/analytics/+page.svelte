<script lang="ts">
  // NOTE: we do actually need this to import the scripts
  import UserLayout from '$components/UserLayout.svelte';
  import LoadingBars from '$components/LoadingBars.svelte';
  import { parse_feedback_data } from '$lib/parser';
  import type { ReportType } from '$lib/chart_utils';

  import {
    startOfMonth,
    endOfMonth,
    format,
    startOfWeek,
    endOfWeek,
    addDays,
  } from 'date-fns';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto, invalidateAll } from '$app/navigation';
  import toast from 'svelte-french-toast';
  import { invoke } from '@tauri-apps/api/tauri';
  import { to_timestamp } from '$root/lib/utils';
  import MonthlyChart from '$components/MonthlyChart.svelte';
  import { generate_report } from '$root/lib/chart_utils';
  import DailyChart from '$components/DailyChart.svelte';
  import WeeklyChart from '$components/WeeklyChart.svelte';
  let loaded: boolean = false;

  let start_date: string = '';
  let end_date: string = '';
  let range: string = 'monthly';
  let data: any = '';

  let no_feedback_data: boolean = false;
  $: range,
    (async () => {
      console.log('RANGE MONITOR: ', range);
      if (custom_range) {
        // NOTE: disable auto date setting
        return;
      }
      const today = new Date();
      if (range === 'daily') {
        // Set start_date to the first day of the current month
        start_date = format(startOfMonth(today), 'yyyy-MM-dd');
        // Set end_date to the last day of the current month
        end_date = format(endOfMonth(today), 'yyyy-MM-dd');
        console.log('DATES: ', { start_date, end_date });
        filter(new Event('click'));
        return;
      }
      if (range === 'monthly') {
        // Set start_date to the first day of the current year
        start_date = format(today, 'yyyy-01-01');
        // Set end_date to the last day of the current year
        end_date = format(today, 'yyyy-12-31');
        filter(new Event('click'));
        return;
      }
      if (range === 'weekly') {
        // Set start_date to the first day of the current year
        start_date = format(today, 'yyyy-01-01');
        // Set end_date to the last day of the current year
        end_date = format(today, 'yyyy-12-31');
        filter(new Event('click'));
        return;
      }
    })();

  let custom_range: boolean = false;

  const toggle_checkbox = (event: Event) => {
    custom_range = (event.target as HTMLInputElement).checked;
  };

  const filter = async (event: Event) => {
    const start = to_timestamp(start_date);
    // NOTE: we will set that the end date ends at 23:59:59
    const end = (
      parseInt(
        to_timestamp(format(addDays(end_date, 1), 'yyyy-MM-dd HH:mm:ss')),
      ) - 1
    ).toString();
    // @ts-ignore
    const hybrid_feedback_data: string = await invoke('get_feedbacks', {
      class: 'hybrid',
      start,
      end,
    }).catch((err: any): string => {
      // WARN: is this a bad idea? idk xD
      // who cares, i'll swallow the error xD
      no_feedback_data = true;
      return '';
    });
    const parsed_data = generate_report(
      range as ReportType,
      hybrid_feedback_data,
    );
    data = parsed_data;
  };

  onMount(async () => {
    if (!$page.data.session) {
      toast.error('Please login first.');
      await goto('/login');
    }
    range = localStorage.getItem('range') ?? 'monthly';
    localStorage.removeItem('range');
    // NOTE: we have to intentionally delay for 100ms
    // to allow the range value change event to propagate
    await new Promise((resolve) => setTimeout(resolve, 100));
    // TEST: for aesthetics we will delay the load for a second xD
    setTimeout(async () => {
      loaded = true;
    }, 1000);
  });
</script>

<UserLayout>
  <div class="flex flex-col h-[calc(100vh-144px)]">
    {#if loaded}
      {#if $page.data.session}
        <div class="w-full">
          <div class="flex flex-row gap-2 float-end mr-14">
            <div class="flex items-center flex-row gap-2">
              <span class="label-text w-full">Custom range</span>
              <input
                type="checkbox"
                class="checkbox checkbox-primary"
                checked={custom_range}
                on:click={toggle_checkbox}
              />
            </div>
            <select class="select select-bordered" bind:value={range}>
              <option value="daily">Daily</option>
              <option value="weekly">Weekly</option>
              <option value="monthly" selected>Monthly</option>
            </select>
            <input
              type="date"
              class="input input-bordered w-1/2"
              readonly={!custom_range}
              bind:value={start_date}
            />
            <input
              type="date"
              class="input input-bordered w-1/2"
              readonly={!custom_range}
              bind:value={end_date}
            />
            <button class="btn btn-primary" on:click={filter}
              >Refresh Data</button
            >
          </div>
        </div>
        {#if !no_feedback_data}
          <div
            class="w-full h-full px-14 py-4 flex flex-col items-center pb-20"
          >
            {#if range === 'daily'}
              <DailyChart {data} />
            {/if}
            {#if range === 'weekly'}
              <WeeklyChart {data} />
            {/if}
            {#if range === 'monthly'}
              <MonthlyChart {data} />
            {/if}
          </div>
        {:else}
          <div
            class="px-14 py-4 flex flex-col items-center justify-center h-full"
          >
            <h1 class="font-bold text-2xl">
              No data to show. Ensure the database is not empty.
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
