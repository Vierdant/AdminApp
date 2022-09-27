<script lang="ts">
    export let authState: boolean = false;

    import { createEventDispatcher } from 'svelte';
    import { fade } from 'svelte/transition';

    import SuccessMark from '../../../assets/icons/checkmark-icon.webp';
    import FailMark from '../../../assets/icons/xmark-icon.webp';

    const dispatch = createEventDispatcher();

    const handleGoBack = () => {
        dispatch('returnRequest');
    }
</script>

<div class="background" transition:fade="{{ duration: 100 }}">
    <div class="screen-center container">
        {#if authState}
            <img src={SuccessMark} alt="logo" height=120 width=120>
            <p>The operation was a success! Please wait for a moment for a redirection.</p>
            
        {:else}
            <img src={FailMark} alt="logo" height=120 width=120>
            <p>Something went wrong! Please make sure that you have entered all required information and no field is displayed in red.</p>
            <button class="btn-appearance btn-light btn-authpop" on:click={handleGoBack}>Return</button>
        {/if}
    </div>
</div>


<style lang="scss">

    .background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 100;
    }

    .container {
        width: 300px;
        background: white;
        border-radius: 10px;

        img {
            padding-top: 1rem;
            display: block;
            margin-left: auto;
            margin-right: auto;
            opacity: 0.8;
        }

        p {
            font-family: Arial, Helvetica, sans-serif;
            display: block;
            margin-left: auto;
            margin-right: auto;
            text-align: center;
            padding-top: 1rem;
            padding-bottom: 1rem;
            max-width: 80%;
            border-top: 1px solid #aeaeae;
        }

        .btn-authpop {
            font-family: Arial;
            display: block;
            margin-left: auto;
            margin-right: auto;
            padding-top: 1rem;
            padding-bottom: 1rem;
            border-top: 1px solid #c5c5c5;
            width: 100%;
            border-radius: 0px;
            border-bottom-left-radius: 2rem;
            border-bottom-right-radius: 2rem;
            margin-top: 1rem;
            font-size: 1.1rem;
            font-weight: 100;
        }
    }
</style>
