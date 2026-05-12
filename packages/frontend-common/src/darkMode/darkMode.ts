export function watchDarkMode() {
    function listener(ev: MediaQueryListEvent | MediaQueryList) {
        const isDark = ev.matches;

        if (isDark) {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
    }

    const matchMedia = window.matchMedia("(prefers-color-scheme: dark)");
    matchMedia.addEventListener("change", listener);
    listener(matchMedia);
    return () => {
        matchMedia.removeEventListener("change", listener);
    };
}
