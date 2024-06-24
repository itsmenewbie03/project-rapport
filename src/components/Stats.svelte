<script>
  import Icon from "./Icon.svelte";
</script>

<div class="stats shadow bg-base-200">
  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="feedback" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Feedbacks</div>
    <div class="stat-value">6.9K</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>

  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="happy" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Postive Feedbacks</div>
    <div class="stat-value">4.2k</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>

  <div class="stat">
    <div class="stat-figure text-secondary">
      <Icon name="sad" _class="inline-block h-8 w-8 stroke-current" />
    </div>
    <div class="stat-title">Negative Feedbacks</div>
    <div class="stat-value">2.7k</div>
    <div class="stat-desc">Jan 1st - Feb 1st</div>
  </div>
</div>
