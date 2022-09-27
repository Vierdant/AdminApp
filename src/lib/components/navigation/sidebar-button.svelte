<script lang="ts">
	import { createEventDispatcher } from "svelte";
    import { user_level } from '$lib/stores/user';
    
    export let title: string;
    export let href: string;
    export let icon: string;
    export let signal: string;
    export let permission: number;

    const dispatch = createEventDispatcher();

    let isSelected = false;

    $: {
        if (signal != title) {
            isSelected = false;
        }
    }
</script>

{#if $user_level >= permission}
    <a href={href} class="sidebar-button" class:selected={isSelected} 
        on:click|preventDefault={() => { isSelected = true; dispatch('selected', title); }}>
        <i class="material-symbols-sharp icon">
            {icon}
        </i>
        <span>{title}</span>
    </a>
{/if}

<style lang="scss">
    .sidebar-button {
        display: flex;
        align-items: center;
        padding: 0.5rem 0.2rem;
        user-select: none;
        max-height: 2.5rem;
        color: var(--text-color);
        text-decoration: none;
        cursor: pointer;

        &::before {
            content: '';
            position: absolute;
            left: 0.8rem;
            max-height: 2.5rem;
            height: 100%;
            width: 0rem;
            background: var(--theme-snd-color);
            transform-origin: bottom;
            transition: ease-out 150ms;
        }
        
        &:hover {
            color: var(--theme-snd-color);

            .icon {
                color: var(--theme-snd-color);
            }
        
            &::before {  
                width: 0.3rem;
            }
        }

        span {
            font-size: 1.2rem;
            font-family: Poppins;
            color: var(--text-color);
        }

        .icon {
            margin-right: 0.7rem;
            opacity: 0.8;
            color: var(--text-color);
        }
    }

    .selected {
        color: var(--theme-snd-color);

        &::before {
            content: '';
            position: absolute;
            left: 0.8rem;
            max-height: 2.5rem;
            height: 100%;
            width: 0.3rem;
            background: var(--theme-snd-color);
            transform-origin: bottom;
            transition: ease-out 150ms;
        }

        .icon {
            color: var(--theme-snd-color);
        }
    }

        
</style>