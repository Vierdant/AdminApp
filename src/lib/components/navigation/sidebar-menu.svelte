<script lang="ts">
    import { slide } from 'svelte/transition'
    import { createEventDispatcher } from 'svelte'
    import { user_level } from '$lib/stores/user';

    export let items: Array<{ href: string, text: string, icon: string }>;
    export let title: string;
    export let icon: string;
    export let signal: string;
    export let permission: number;

    const dispatch = createEventDispatcher();

    let isOpen = false;
    let isSelected = false;

    $: {
        if (signal != title) {
            isSelected = false;
        }
    }
</script>

{#if $user_level >= permission}
    <div class="sidebar-menu">
        <div class="title" on:click={() => isOpen = !isOpen} class:selected={isSelected}>
            <i class="material-symbols-sharp icon">
                {icon}
            </i>
            <span>{title}</span>
            {#if isOpen}
                <i class="material-symbols-sharp arrow">
                    expand_less
                </i>
            {:else}
                <i class="material-symbols-sharp arrow">
                    navigate_next
                </i>
            {/if}
        </div>
        {#if isOpen}
            <div class="items-container" transition:slide>
                {#each items as item}
                    <a href={item.href} on:click|preventDefault={() => { isSelected = true; dispatch('selected', title); }}>
                        <div class="item">
                            <i class="material-symbols-sharp">
                                {item.icon}
                            </i>
                            <span>{item.text}</span>
                        </div>
                    </a>
                {/each}
            </div>
        {/if}
    </div>
{/if}

<style lang="scss">

    .title {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 0.2rem;
        user-select: none;
        max-height: 2.5rem;
        color: var(--text-color);
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
            margin-right: 0.2rem;
            opacity: 0.8;
        }

        .arrow {
            font-size: 1.2rem;
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
    }

    .items-container {
        padding: 0rem 1rem 0.5rem 1rem;
        margin-left: 0.9rem;
        text-overflow: clip;
        border-left: 1px solid var(--theme-snd-color);

        a {
            text-decoration: none;
            color: var(--text-color);
        }
    }

    .item {
        display: flex;
        align-items: center;
        padding: 0.5rem 0rem;
        user-select: none;
        cursor: pointer;

        &:hover {
            color: var(--theme-snd-color);

            ::after {
                content: "";
                position: absolute;
                left: 1.7rem;
                margin-top: 0.4rem;
                height: 0.5rem;
                width: 0.5rem;
                border-radius: 50%;
                background: var(--theme-snd-color);

            }
        }
        
        i {
            font-size: 1.2rem;
            margin-right: 0.5rem;
            opacity: 0.8;

        }

        span {
            font-size: 0.9rem;
            font-family: Poppins;
        }
    }


</style>
