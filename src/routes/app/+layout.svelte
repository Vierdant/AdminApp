<script lang="ts">
    import SidebarButton from '$lib/components/navigation/sidebar-button.svelte';
    import SidebarMenu from '$lib/components/navigation/sidebar-menu.svelte';
    import ThemeButton from '$lib/components/misc/theme-button.svelte';
    import logoImg from "$lib/../assets/clean_logo.webp";
    import { user_name, user_position } from '$lib/stores/user'
    import { onMount } from 'svelte';
	import { ImageLib } from '$lib/image';
	import { POSITIONS } from '$lib/stores/positions';

    let userPicture: string;
    let userPositionDisplay: string;

    const DashboardItems = [
        { href: "/app/dashboard", text: "eCommerce", icon: "upload_file" },
        { href: "/app/dashboard", text: "Systems", icon: "arrow_right" },
        { href: "/app/dashboard", text: "Databases", icon: "arrow_right" }
    ]

    let signal = "null";
    /**
     * signals the switch to all navigation components
     * so they turn off their selected state
     */
    const signalSwitch = (data: any) => {
        signal = data.detail;
    }

    onMount(() => {
        userPicture = ImageLib.getProfileImageUrl();

        userPositionDisplay = POSITIONS.get($user_position)!.display_name;

    });

</script>

<!-- Side bar -->
<aside>
    <h2 id="sidebar-title">Menu</h2>
    <div class="sidebar-content">
        <SidebarMenu permission={0} title="Dashboard" icon="space_dashboard" items={DashboardItems} {signal} on:selected={signalSwitch}/>
        <SidebarButton permission={0} title="Settings" icon="settings" href="/app/dashboard" {signal} on:selected={signalSwitch}/>
    </div>
</aside>

<!-- Top bar -->
<div class="top-bar">
    <div class="top-bar-left">
        <div class="logo">
            <img src={logoImg} alt="logo" height=50 width=50>
            <span>Returned Admin</span>
        </div>
    </div>
    <div class="top-bar-right">
        <ThemeButton></ThemeButton>
        <div class="user">
            <span>{$user_name}</span>
            <span>{userPositionDisplay}</span>
        </div>
        <div class="profile">
            <img class="image" src={userPicture} alt="user" height=40 width=40>
        </div>
    </div>
</div>

<slot></slot>

<style lang="scss">

    aside {
        background: var(--theme-color);
        float: left;
        position: relative;
        min-height: 100vh;
        // width of the sidebar
        width: 12rem;
        border-bottom-right-radius: 0.5em;
        box-shadow: 2px 1px 12px 1px rgba(107, 107, 107, 0.728);
    }

    #sidebar-title {
        text-align: center;
        margin-top: 1rem;
        margin-bottom: 1rem;
        padding: 2rem 0 2rem 0;
        color: var(--text-color);

        &::after {
            content: "";
            position: absolute;
            height: 0.2rem;
            width: 2rem;
            background-color: var(--theme-snd-color);
            left: 4rem;
            top: 5rem;
        }
    }

    .sidebar-content {
        margin-left: 1rem;
        margin-right: 1rem;
    }

    .top-bar {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        position: relative;
        align-items: center;
        height: 50px;
        background: var(--theme-color);
        color: var(--text-color);
        padding: 0 20px;

        span {
            font-size: 0.8rem;
            color: var(--text-color);
        }

        .top-bar-left {
            display: flex;
            flex-direction: row;
            align-items: center;

            .logo {
                display: flex;
                flex-direction: row;
                align-items: center;

                img {
                    margin-right: 10px;
                    user-select: none;
                    filter: brightness(var(--logo-brightness-value));
                }
            }
        }

        .top-bar-right {
            display: flex;
            flex-direction: row;
            align-items: center;
            margin-right: 2rem;

            .user {
                display: flex;
                flex-direction: column;
                align-items: flex-end;
            }

            .profile {
                img {
                    border-radius: 50%;
                    margin-left: 1rem;
                    margin-top: 5px;
                }
            }
        }
    }
</style>