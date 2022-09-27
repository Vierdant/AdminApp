<script lang="ts">
    import { fade } from 'svelte/transition';
</script>

<span>Gradiant Control</span>

<div class="screen-center container" in:fade>
    <slot></slot>
</div>

<style lang="scss">

    span {
        opacity: 0;
        user-select: none;
    }

    .container {
        width: 400px;
        background: white;
        border-radius: 10px;
        box-shadow: 10px 10px 15px rgba(0,0,0,0.05);
    }

</style>