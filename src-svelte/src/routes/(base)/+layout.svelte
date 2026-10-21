<script lang="ts">

import type { LayoutProps } from "./$types";

import WithHeaderFooterLayout from "$lib/components/04-layout/WithHeaderFooterLayout.svelte";


let { children }: LayoutProps = $props()

</script>


<WithHeaderFooterLayout>
  {@render children?.()}
</WithHeaderFooterLayout>
