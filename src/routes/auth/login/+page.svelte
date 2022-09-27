<script lang="ts">
    import { invoke } from  '@tauri-apps/api/tauri';
    import { ValidateString } from '$lib/validate';
    import AuthPopup from '$lib/components/popup/auth.svelte';
    import TextField from '$lib/components/txt-field/errorable.svelte';
    import ForgotPassword from '$lib/components/misc/forgot-password.svelte'
    import { user_name, user_position, user_status, user_discord, user_image, user_level } from '$lib/stores/user';
    import { goto } from '$app/navigation';
	import { POSITIONS } from '$lib/stores/positions';
    
    let username: string;
    let password: string;

    // error box
    let global_error: boolean = false;

    let goodValidation: boolean = false;
    let goodResponse: boolean = false;
    let showAuthModal: boolean = false;
    let showLoader: boolean = false;

    const setUserData = async () => {
        // update the name
        user_name.set(username);

        invoke('user_get_data', { username: username }).then((result) => {
            let data = JSON.parse(JSON.stringify(result));

            user_position.set(data.position);
            user_status.set(data.status);
            user_discord.set(data.discord);
            user_image.set(data.image);
            user_level.set(POSITIONS.get(data.position)!.level);
        })
        .catch((err) => {
            console.log(err);
        });
    }

    const submitCredentials = async () => {
        if (!goodValidation) {
            goodResponse = false;
            global_error = true;
            showAuthModal = true;
            return;
        }
        
        showLoader = true;

        const response: string = await invoke('authenticate_user', {
            username,
            password
        });

        if (response === 'good') {
            goodResponse = true;
            showAuthModal = true;
            showLoader = false;
            console.log(username + " " + $user_name)
            
            await setUserData();

            console.log("After set: " + $user_name)
            setTimeout(() => {
                goto('/app/dashboard');
                
            }, 2000);
        } else {
            goodResponse = false;
            global_error = true;
            showAuthModal = true;
            showLoader = false;
        }
    }

    $: {
        goodValidation = ValidateString.isDefinedAndNotEmpty(username) && ValidateString.isDefinedAndNotEmpty(password);
        
        global_error = false;
    }

    /*
    invoke("authenticate_user", { username: "Vierdant", password: "bitch" }).then((result) => {
        state = result;
    })
    */
</script>

<h2>Login</h2>
<div class="content-container">
    <TextField label="Username" bind:value={username} 
        tooltip="Your account's username"/>
    <TextField label="Password" bind:value={password} type="password"
        tooltip="Your account's password, not the one-time use token"/>
    <ForgotPassword>Forgot Password?</ForgotPassword>
    {#if showLoader}
        <div class="loader-icon"></div>
    {:else}
        <button on:click={submitCredentials} class="btn-primary submit-btn">Login</button>
    {/if}
    <div class="extra-link">
        New member? <a href="register">Verify</a>
    </div>
</div>

{#if showAuthModal}
    <AuthPopup authState={goodResponse} on:returnRequest={() => showAuthModal = false}/>
{/if}

<style lang="scss" src="../auth-style.scss">
</style>