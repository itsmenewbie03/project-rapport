<script lang="ts">
  import { Line } from 'svelte-chartjs';
  import { Colors } from 'chart.js';

  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
    Filler,
  } from 'chart.js';

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
    Filler,
    Colors,
  );

  // NOTE: we need to cast it to
  // any so TS would shutup xD
  export let data;

  const opts = {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      y: {
        min: 0.0,
        max: 5.0,
      },
    },
    plugins: {
      title: {
        display: true,
        text: 'Monthly Traditional Feedback Data Chart',
      },
      legend: {
        labels: {
          usePointStyle: true,
        },
      },
    },
  };
</script>

<Line {data} options={opts} />
