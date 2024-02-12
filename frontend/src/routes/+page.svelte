<script>
	import Counter from './Counter.svelte';
	import Table from './Table.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="" />
</svelte:head>

<section>
	<h1>
		<span class="welcome">
			Europe is Hot
		</span>
	</h1>

	<Table />
</section>

<style>
	section {
		display: contents;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 4096) 0;
	}
</style>
