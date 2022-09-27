<!-- svelte-ignore a11y-label-has-associated-control -->



<script lang="ts">

    /**
     * Errorable text field is meant to give the user the ability to error if a value does not match
     * a desired input
     */

    export let error: boolean = false;
    export let tooltip: string = "";
    export let type: "text" | "password" = "text";
    export let label: string;
    export let value = "";
</script>


{#if type === "password"}
    <div class="txt_field" class:error-field={error} data-tool-tip={tooltip}>
        <input type="password" title="" bind:value={value} required>
        <span></span>
        <label>{label}</label>
    </div>
{:else}
    <div class="txt_field" class:error-field={error} data-tool-tip={tooltip}>
        <input type="text" title="" bind:value={value} required>
        <span></span>
        <label>{label}</label>
    </div>
{/if}

<style lang="scss">

    .txt_field {
        position: relative;
        border-bottom: 2px solid #adadad;
        margin: 30px 0;

        &::after {
            content: attr(data-tool-tip);
            display: block;
            position: absolute;
            background-color: #363636;
            opacity: 0.9;
            padding: 1em 3em;
            color: white;
            font-family: Arial, Helvetica, sans-serif;
            border-radius: 5px;
            font-size: .8em;
            top: 100%;
            left: 25%;
            margin-top: .3em;
            white-space: pre-wrap;
            z-index: 100;
            transform: scale(0);
            transform-origin: top;
            transition: transform ease-out 150ms;
        }

        &:hover::after {
            transform: scale(1);
            transition-delay: 0.5s;
        }

        input {
            width: 100%;
            padding: 0 5px;
            height: 40px;
            font-size: 16px;
            border: none;
            background: none;
            outline: none;

            &:focus ~ label,
            &:valid ~ label {
                top: -5px;
                color: #2691d9;
            }

            &:focus ~ span::before,
            &:valid ~ span::before {
                width: 100%;
            }
        }

        label {
            position: absolute;
            top: 50%;
            left: 5px;
            color: var(--text-muted-color);
            transform: translateY(-50%);
            font-size: 16px;
            pointer-events: none;
            transition: .5s;
            font-family: Arial, Helvetica, sans-serif;
        }

        span::before {
            content: '';
            position: absolute;
            top: 40px;
            left: 0;
            width: 0%;
            height: 2px;
            background: #2691d9;
            transition: .5s;
        }
    }

    .error-field {
        border-bottom: 2px solid var(--text-bad-color);

        label {
            color: var(--text-bad-color);
        }

        span::before {
            background: var(--text-bad-color);
        }

        input:focus ~ label,
        input:valid ~ label {
            color: var(--text-bad-color);
        }

        input:focus ~ span::before,
        input:valid ~ span::before {
            width: 100%;
        }
    }
</style>