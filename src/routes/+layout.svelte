<script lang="ts">
	import '@fontsource/comfortaa';
	import '@fontsource/open-sans';
	import '@fontsource/roboto';
	import '@fontsource/source-code-pro';

	import '../app.css';

	let { children, title = $bindable('Bizarre Friends'), ...props } = $props();

	let menu: HTMLElement;
	function toggleMenu() {
		menu?.classList.toggle('-translate-y-full');
	}
</script>

<div
	class="sticky top-0 left-0 right-0 flex p-1 z-50 align-middle bg-primary-900 text-slate-200 fill-slate-200"
>
	<button type="button" class="flex px-1" onclick={toggleMenu}>
		<svg
			class="fill-slate-300 hover:fill-slate-100 w-[32px] h-[32px]"
			id="menu-icon"
			version="1.1"
			role="img"
			aria-label="[title + description]"
		>
			<title>Menu</title>
			<description>Open or close the menui</description>
			<path
				d="M26,16c0,1.104-0.896,2-2,2H8c-1.104,0-2-0.896-2-2s0.896-2,2-2h16C25.104,14,26,14.896,26,16z"
				id="XMLID_314_"
			/><path
				d="M26,8c0,1.104-0.896,2-2,2H8c-1.104,0-2-0.896-2-2s0.896-2,2-2h16C25.104,6,26,6.896,26,8z"
				id="XMLID_315_"
			/><path
				d="M26,24c0,1.104-0.896,2-2,2H8c-1.104,0-2-0.896-2-2s0.896-2,2-2h16C25.104,22,26,22.896,26,24z"
				id="XMLID_316_"
			/></svg
		>
	</button>
	<h1
		class="flex flex-auto m-auto"
		contenteditable="false"
		bind:innerHTML={title}
	>
		&nbsp;
	</h1>
</div>
<nav
	bind:this={menu}
	class="absolute flex flex-col left-0 z-30 bg-secondary-900 text-gray-200 transition ease-in-out origin-top-left delay-150 py-1 w-full -translate-y-full"
>
	<ul class="flex flex-col w-full">
		<li class="p-2 hover:bg-secondary-800"><a href="/">Home</a></li>
		<li class="p-2 hover:bg-secondary-800"><a href="/">Page 1</a></li>
		<li class="p-2 hover:bg-secondary-800"><a href="/">Page 2</a></li>
		<li class="p-2 hover:bg-secondary-800"><a href="/">Page 3</a></li>
	</ul>
</nav>

<div class="relative z-0 p-4">
	{@render children?.()}
</div>
