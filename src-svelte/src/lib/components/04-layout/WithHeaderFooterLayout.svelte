<script lang="ts">

import type { Snippet } from "svelte";

import Header from "$lib/components/01-widget/Header/Header.svelte";


interface Props {
  children: Snippet;
}

let { children }: Props = $props();

</script>


<Header/>
{@render children()}
