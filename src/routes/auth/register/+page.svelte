<script lang="ts">
    import { fade } from 'svelte/transition';
    import { ValidateString } from '$lib/validate';
    import { invoke } from '@tauri-apps/api/tauri';
    import AuthPopup from '$lib/components/popup/auth.svelte';
    import TextField from '$lib/components/txt-field/errorable.svelte';
    import { goto } from '$app/navigation';

    // info box
    let username: string;
    let email: string;
    let token: string;
    let password: string;
    let confirm_password: string;
    
    // error box
    let email_error: boolean = false;
    let password_error: boolean = false;
    let confirm_password_error: boolean = false;
    let global_error: boolean = false;

    let goodValidation: boolean = false;
    let goodResponse: boolean = false;
    let showAuthModal: boolean = false;
    let showLoader: boolean = false;

    const submitCredentials = async () => {
        if (!goodValidation) {
            goodResponse = false;
            global_error = true;
            showAuthModal = true;
            return;
        }

        showLoader = true;

        const response: string = await invoke('register_user', {
            username,
            email,
            password,
            token
        });

        if (response === 'good') {
            goodResponse = true;
            showAuthModal = true;
            showLoader = false;
            setTimeout(() => {
                goto('/auth/login');
                
            }, 4000);
        } else {
            goodResponse = false;
            global_error = true;
            showAuthModal = true;
            showLoader = false;
        }
    }

    $: {
        email_error = ValidateString.isDefinedOrEmpty(email) 
                && !ValidateString.isEmail(email);
        password_error = ValidateString.isDefinedOrEmpty(password) 
                && !ValidateString.isGoodPassword(password);
        confirm_password_error = ValidateString.isDefinedOrEmpty(confirm_password) 
                && confirm_password !== password;

        goodValidation = !email_error && !password_error && !confirm_password_error 
                && ValidateString.isDefinedAndNotEmpty(username) && ValidateString.isDefinedAndNotEmpty(token);
        
        global_error = false;
    }
</script>


<h2>Register</h2>
<div class="content-container">
    <TextField label="Username" bind:value={username} error={global_error} 
        tooltip="The username given by an admin"/>
    <TextField label="Token" bind:value={token} error={global_error} 
        tooltip="The token given by an admin"/>
    <TextField label="Email" bind:value={email} error={email_error || global_error}
        tooltip="The email you wish to associate your account with"/>
    <TextField label="Password" bind:value={password} error={password_error || global_error} type="password"
        tooltip="8-18 characters, at least 1 uppercase letter, 1 lowercase letter, 1 number and 1 special character"/>
    <TextField label="Confirm Password" bind:value={confirm_password} error={confirm_password_error || global_error} type="password"
        tooltip="Confirm your account's password"/>
    {#if showLoader}
        <div class="loader-icon"></div>
    {:else}
        <button on:click={submitCredentials} class="btn-primary submit-btn">Register</button>
    {/if}
    <div class="extra-link">
      Already a member? <a href="login">Login</a>
    </div>
</div>
    
{#if showAuthModal}
    <AuthPopup authState={goodResponse} on:returnRequest={() => showAuthModal = false}/>
{/if}


<style lang="scss" src="../auth-style.scss"></style>