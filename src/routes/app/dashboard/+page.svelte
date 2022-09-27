<script lang="ts">
    import { user_name, user_position, user_level } from '$lib/stores/user'
    import logoImg from "$lib/../assets/clean_logo.webp";
    
</script>

<div class="screen-center">
    <img src={logoImg} alt="logo" height=200 width=200>
    <h2>Welcome to Returned Admin</h2>
    <span>Please select a page to start</span>
    <span>Your level is: {$user_level}</span>
</div>

<style lang="scss">
    div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        text-align: center;
    }

    h2 {
        font-family: Arial, Helvetica, sans-serif;
        font-size: 1.5rem;
        color: var(--text-color);
    }

    span {
        font-family: Arial, Helvetica, sans-serif;
        font-size: 1rem;
        color: var(--text-color);
    }
</style>