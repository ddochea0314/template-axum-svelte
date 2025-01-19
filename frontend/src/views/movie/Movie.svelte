<script lang="ts">
  import { onMount } from "svelte";
  import Nav from "../../lib/Nav.svelte";

  let data : { 
    title : string,
    director : string,
    year : number,
  }[] = [];

  onMount(async () => {
    console.log("Movie component mounted");
    const res = await fetch("/api/movie");
    data = await res.json();
  });
</script>
<Nav />
<main>
<h1>Movies</h1>

<ul>
  {#each data as movie}
    <li>{movie.title}</li>
  {/each}
</ul>
</main>