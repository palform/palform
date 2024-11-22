<script lang="ts">
    import logo from "../../assets/logo_type.svg?raw";
    import NavItem from "./NavItem.svelte";
</script>

<nav class="fixed w-full flex justify-center pt-2 z-10">
    <div class="px-6 md:px-12 xl:px-20 max-w-7xl w-full">
        <div
            class="h-12 px-8 flex justify-between items-center bg-slate-100/80 dark:bg-slate-800/80 backdrop-blur-lg rounded-full border border-slate-400/40"
        >
            <p class="transition-transform hover:scale-105 active:scale-95">
                <a href="/#">
                    <svg class="h-7" viewBox="0 0 304.07928 85.568403">
                        {@html logo}
                    </svg>
                </a>
            </p>
            <ul
                class="flex items-center gap-4 sm:gap-8 dark:text-gray-200 font-display font-light text-primary-800"
            >
                <NavItem
                    subItems={[
                        {
                            title: "End-to-end encryption",
                            description:
                                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                            href: "/features/encryption",
                        },
                    ]}
                >
                    Features
                </NavItem>
                <NavItem href="/templates">Templates</NavItem>
                <NavItem href="/#pricing">Pricing</NavItem>
                <NavItem href="https://dash.palform.app/auth/login">
                    Sign in
                </NavItem>
            </ul>
        </div>
    </div>
</nav>
