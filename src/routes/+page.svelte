<script lang="ts">
	import { onMount } from 'svelte';

	let { title = $bindable() }: { title: string } = $props();
	title = 'Test';
</script>

<div class="items-center content-center">
	<h1>Home</h1>
	<h2>Heading 2</h2>
	<h3>Heading 3</h3>
	<h4>Heading 4</h4>

	<div class="flex h-full place-content-center">
		<a href="https://vitejs.dev" target="_blank">
			<img src="/vite.svg" class="h-16 p-2" alt="Vite Logo" />
		</a>
		<a href="https://tauri.app" target="_blank">
			<img src="/tauri.svg" class="h-16 p-2" alt="Tauri Logo" />
		</a>
		<a href="https://kit.svelte.dev" target="_blank">
			<img src="/svelte.svg" class="h-16 p-2" alt="SvelteKit Logo" />
		</a>
	</div>

	<p>
		Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas lorem
		mauris, aliquet et elementum id, pretium sed diam. In hac habitasse
		platea dictumst. Curabitur felis risus, tincidunt at turpis a, aliquam
		rhoncus ante. Aenean tortor felis, vehicula eu magna rutrum, gravida
		fringilla ante. Pellentesque elementum, tortor quis cursus venenatis,
		nunc enim auctor ante, nec maximus mauris ipsum eu ipsum. Nulla at diam
		urna. Nullam laoreet finibus tristique. Donec cursus laoreet ex et
		pharetra. Nunc interdum sem ex, in dictum lorem sagittis ut. Maecenas
		neque augue, volutpat quis ligula quis, ultricies cursus nisl. Cras ante
		est, ultricies sit amet ipsum fermentum, tincidunt pulvinar orci. Donec
		porta odio id ligula rhoncus consequat. Nulla posuere quis quam nec
		consectetur.
	</p>

	<p>
		Maecenas nec nisl a justo egestas volutpat. Nunc luctus ante tortor,
		eget aliquam quam facilisis eget. Praesent faucibus ut elit in
		facilisis. Nunc a lacus augue. In rhoncus eu justo eget tempus. Etiam
		euismod cursus dui, et ultrices felis aliquet ac. Nam ac enim lacus. In
		hac habitasse platea dictumst.
	</p>

	<p>
		Nunc lorem sapien, efficitur in justo eget, placerat sodales arcu.
		Quisque eu pellentesque leo. Class aptent taciti sociosqu ad litora
		torquent per conubia nostra, per inceptos himenaeos. Nulla facilisi.
		Interdum et malesuada fames ac ante ipsum primis in faucibus. In sit
		amet scelerisque est. Suspendisse vehicula eget ex sed porta. Integer
		lorem tortor, cursus nec nunc a, bibendum iaculis ipsum. Nam id
		tincidunt magna. Aliquam nec lectus hendrerit, facilisis eros in,
		imperdiet elit. Quisque laoreet lobortis eleifend. Nullam ligula diam,
		accumsan eu magna ut, accumsan rutrum arcu. Vestibulum at arcu at ipsum
		euismod gravida. Phasellus ultricies scelerisque diam, ac mollis ante
		tempus sit amet. Sed auctor sem id nisi congue accumsan. Fusce non
		tempor velit, sit amet interdum ligula.
	</p>

	<p>
		Mauris ultricies, urna quis fermentum lobortis, velit urna porta neque,
		eu efficitur ex risus in enim. Nunc orci mauris, elementum in lacus ac,
		vehicula interdum ipsum. Vestibulum et molestie mauris, in tincidunt
		lorem. Vestibulum faucibus ullamcorper dolor vel lacinia. In aliquam
		feugiat orci, sed blandit erat mattis in. Maecenas eu congue lacus. Cras
		nisl leo, egestas maximus urna quis, cursus malesuada elit. Donec
		ullamcorper ornare lacus, ut maximus nisi finibus at. Nulla volutpat
		dignissim eros et condimentum. Fusce vulputate nisl ac mauris egestas,
		ut ultrices sapien blandit. Proin pretium mi sit amet lacus feugiat, id
		imperdiet massa imperdiet. Fusce magna felis, finibus quis nisi ac,
		venenatis lobortis ante. Fusce tincidunt mi sed ex feugiat dapibus.
	</p>

	<p>
		Donec vel tristique lectus, vel congue justo. Praesent interdum nisi vel
		cursus consequat. Nulla facilisi. Integer gravida ante id iaculis
		sollicitudin. Aliquam facilisis porta enim et facilisis. Donec nec
		condimentum quam. Aliquam luctus augue nec fermentum finibus. Nulla
		ultricies nisl est, et luctus mauris malesuada vehicula. Maecenas
		cursus, ex id accumsan accumsan, sapien purus faucibus lorem, non
		consequat felis turpis ac ante. Quisque in arcu ullamcorper, elementum
		dui eu, tempus ligula. Cras lorem lacus, tempus id tellus blandit,
		congue finibus leo.
	</p>
</div>
