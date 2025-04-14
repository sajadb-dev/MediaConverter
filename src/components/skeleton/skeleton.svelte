<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";

	type $$Props = HTMLAttributes<HTMLDivElement>;
    let { classnames } = $props();

</script>

<div class="bg-[var(--outline)] animate-pulse rounded-md {classnames}"></div>