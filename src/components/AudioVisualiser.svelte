<script>
  export let data = [];
  export let width = 300;
  export let height = 100;
  export let lineColor = "#3498db";
  export let lineWidth = 2;

  $: points = data
    .map((value, index) => {
      const x = (index / (data.length - 1)) * width;
      const y =
        height -
        ((value - Math.min(...data)) /
          (Math.max(...data) - Math.min(...data))) *
          height;
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
