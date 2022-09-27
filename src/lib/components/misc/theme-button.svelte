<script lang="ts">

	let darkMode = false;

	$: {
		if(darkMode || !darkMode) {
			//document.body.classList.toggle('dark-theme-variables');
		}
	}
</script>

<div>
	<input type="checkbox" class="checkbox" id="checkbox" bind:checked={darkMode} />
	<label for="checkbox" class="label">
		<span class="material-symbols-sharp icon">
			{#if darkMode}
				clear_day
			{:else}
				dark_mode
			{/if}
		</span>
	</label>
</div>

<style lang="scss">
	.checkbox {
		opacity: 0;
		position: absolute;
	}

	.label {
		width: 25px;
		height: 8px;
		background-color: var(--theme-color);
		filter: brightness(.85);
		display: flex;
		border-radius:50px;
		align-items: center;
		justify-content: space-between;
		padding: 5px;
		margin-right: 2rem;
		position: relative;
		transform: scale(1.5);
	}

	.icon {
		position: absolute;
		color: var(--theme-snd-color);
		font-size: 1rem;
		bottom: 1px;
		left: 2.5px;
		border-radius: 50%;
		transition: transform 0.2s linear;
		user-select: none;
	}

	/*  target the elemenent after the label*/
	.checkbox:checked + .label .icon {
		color: var(--text-color);
		transform: translateX(15px);
	}
</style>