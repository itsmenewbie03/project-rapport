<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from './Icon.svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { stats_parser } from '$lib/parser';
  let parsed_data: Record<string, number> = {};
  onMount(async () => {
    const hybrid_feedback_data: string = await invoke('get_feedbacks', {
      class: 'hybrid',
    });
    parsed_data = stats_parser(hybrid_feedback_data);
  });
</script>

<div class="stats shadow bg-base-200">
  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="feedback" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Feedbacks</div>
    <div class="stat-value">{parsed_data['total'] ?? 0}</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>

  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="happy" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Postive Feedbacks</div>
    <div class="stat-value">{parsed_data['positive'] ?? 0}</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>

  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="sad" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Negative Feedbacks</div>
    <div class="stat-value">{parsed_data['negative'] ?? 0}</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>
</div>
