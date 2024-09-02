import { navigate } from "svelte-routing";

const elementIsAnchor = (e: HTMLElement): e is HTMLAnchorElement => {
	return e.tagName === "A";
};

export function navigateEvent(e: MouseEvent | CustomEvent<any>) {
	e.preventDefault();
	let target = e.target as HTMLElement;
	while (!elementIsAnchor(target)) {
		if (target.parentElement === null) {
			throw new Error("Could not find <a> link target");
		}
		target = target.parentElement;
	}

	const url = new URL(target.href);
	navigate(url.pathname);
}
