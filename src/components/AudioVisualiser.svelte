<script>
  export let data = [];
  export let width = 300;
  export let height = 100;
  export let lineColor = "#3498db";
  export let lineWidth = 2;

  if (data.length === 0) {
    data = Array.from({ length: 1024 }).fill(0);
  }

  $: points = data
    .map((value, index) => {
      const x = (index / (data.length - 1)) * width;
      const y = 50 - value * 100;
      return `${x},${y}`;
    })
    .join(" ");
</script>

<svg {width} {height}>
  <polyline {points} fill="none" stroke={lineColor} stroke-width={lineWidth} />
</svg>

<style>
  svg {
    display: block;
    margin: 0 auto;
  }
</style>
