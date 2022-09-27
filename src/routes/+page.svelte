<script lang="ts">
    import { fly, fade } from 'svelte/transition';
    import { invoke } from '@tauri-apps/api/tauri';
    import { goto } from '$app/navigation';
    import logoImg from "../assets/clean_logo.webp";
    import { onMount } from 'svelte';

    import { arrayPositionsToMap } from "$lib/stores/positions";
    
    let positionsStatus: boolean = false;

    let goPermission: boolean = false;

    /**
     * Load the positions entries from the database
     */
    async function loadPositions() {
        let result = await invoke('position_get_all_positions').catch((err) => {
            console.log("error happened help")
            console.log(err);
        });
        let data = JSON.parse(JSON.stringify(result));

        arrayPositionsToMap(data);
        positionsStatus = true;
    }

    onMount(() => {
        loadPositions();
    });

    let outro: boolean = true;

    $: {
        if (positionsStatus) {
            goPermission = true;
        }
    }
</script>

<i>Gradiant Control</i>

<div class="screen-center container">
    <div class="content-contaienr">
        {#if outro}
            <img src={logoImg} alt="logo" height=200 width=200 
            in:fade="{{ duration: 1000 }}">
            {#if goPermission}
                <span 
                in:fade="{{ duration: 1000 }}"
                out:fly="{{ x: 100, duration: 1000 }}" 
                on:click="{() => outro = false}" 
                on:outroend="{() => goto("auth/login")}" class="btn-appearance btn">Go</span>
            {/if}
        {/if}    
    </div>
</div>

<style lang="scss">

    .btn {
        background-color: aliceblue;
        border: 1px solid aliceblue;
        color: black;
        transition: .5s;
    } 

    i {
        opacity: 0;
        user-select: none;
    }

    .container {
        width: 250px;
        padding: 10px 10px 50px 10px;
        border-radius: 20px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        text-align: center;
    }

    .content-contaienr {
        margin-top: 20px;

        span {
            display: inline-block;

            &:hover {
                animation: shake 0.82s cubic-bezier(.36,.07,.19,.97) both;
                transform: translate3d(0, 0, 0);
                perspective: 1000px;
            }
        }
    }

    

    @keyframes shake {
    10%, 90% {
        transform: translate3d(-1px, 0, 0);
    }
    20%, 80% {
        transform: translate3d(2px, 0, 0);
    }
    30%, 50%, 70% {
        transform: translate3d(-2px, 0, 0);
    }
    40%, 60% {
        transform: translate3d(4px, 0, 0);
    }
    }
</style>