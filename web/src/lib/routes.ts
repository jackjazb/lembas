import IconApple from "$lib/components/icons/IconApple.svelte";
import IconPackage from "$lib/components/icons/IconPackage.svelte";
import IconClipboard from "$lib/components/icons/IconClipboard.svelte";
import type { ComponentType } from "svelte";

export interface Route {
	url: string;
	title: string;
	icon: ComponentType;
}

export const ROUTES: Route[] = [
	{
		url: "/recipes",
		title: "My Recipes",
		icon: IconApple,
	},
	{
		url: "/mealplan",
		title: "Meal Plan",
		icon: IconClipboard,
	},
	{
		url: "/ingredients",
		title: "Ingredients",
		icon: IconPackage,
	},
];

/** Takes a page path and resolves a route, returning undefined if no route matches. */
export function routeFromURL(url: string | null): Route | undefined {
	if (!url || !ROUTES.map(route => route.url).includes(url)) {
		return undefined;
	}
	return ROUTES.find(route => route.url === url);
}