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
                    desktopOnly
                    subItems={[
                        {
                            title: "End-to-end encryption",
                            description:
                                "No-one can see your responses. Not even us. Keep your data more secure than ever with Palform.",
                            href: "/features/encryption",
                        },
                        {
                            title: "Open source",
                            description:
                                "Accessible and easily auditable code licensed under AGPL. All development is done in public.",
                            href: "https://github.com/palform/palform",
                        },
                        {
                            title: "Customisable branching",
                            description:
                                "Personalise your forms with recall and branching, keeping your users super engaged throughout.",
                            href: "https://docs.palform.app/forms/branching/",
                        },
                        {
                            title: "Response notifications",
                            description:
                                "Connect your app to a range of services using webhook and email notifications",
                            href: "https://docs.palform.app/forms/notifications/",
                        },
                    ]}
                >
                    Features
                </NavItem>
                <NavItem href="/templates" desktopOnly>Templates</NavItem>
                <NavItem href="/#pricing" desktopOnly>Pricing</NavItem>
                <NavItem href="https://dash.palform.app/auth/login">
                    Sign in
                </NavItem>
            </ul>
        </div>
    </div>
</nav>
