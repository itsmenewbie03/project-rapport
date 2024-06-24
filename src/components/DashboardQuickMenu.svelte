<script lang="ts">
  import Icon from "./Icon.svelte";
</script>

<ul class="menu bg-base-200 w-56 rounded-box">
  <li class="menu-title">Quick Actions</li>
  <li>
    <a href="/feedback/consent">
      <Icon name="start" _class="size-5" />
      Start Feedback
    </a>
  </li>
  <li>
    <a href="/configurations">
      <Icon name="configurations" _class="size-5" />
      Configure System
    </a>
  </li>
  <li>
    <a href="/analytics">
      <Icon name="analytics" _class="size-5" />
      View Analytics
    </a>
  </li>
</ul>
