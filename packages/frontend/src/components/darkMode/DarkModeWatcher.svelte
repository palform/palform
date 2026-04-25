<script lang="ts">
    $effect(() => {
        function darkModeListener(ev: MediaQueryListEvent | MediaQueryList) {
            const isDark = ev.matches;

            if (isDark) {
                document.documentElement.classList.add("dark");
            } else {
                document.documentElement.classList.remove("dark");
            }
        }

        const matchMedia = window.matchMedia("(prefers-color-scheme: dark)");
        matchMedia.addEventListener("change", darkModeListener);
        darkModeListener(matchMedia);
        return () => {
            matchMedia.removeEventListener("change", darkModeListener);
        };
    });
</script>
