import type { ComponentProps } from "svelte";
import { writable, type Writable } from "svelte/store";
import TopBar from "$lib/components/TopBar.svelte";

/** Allows the contents of the top navigation bar to be determined per-route. */
export const topBarProps: Writable<ComponentProps<TopBar>> = writable({
	title: ''
});

export enum ToastType {
	Info,
	Error,
	Success
}

export interface Toast {
	message: string;
	type: ToastType;
}

/** Maintains a global toast notifications. */
export const toast: Writable<Toast | undefined> = writable(undefined);

export function addToast(message: string, type: ToastType) {
	toast.set({ message, type });
	setTimeout(() => {
		removeToast();
	}, 2000);
}

export function removeToast() {
	toast.set(undefined);
}