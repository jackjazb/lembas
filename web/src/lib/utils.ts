/** Takes a page path and resolves a route, returning undefined if no route matches. */
export function getBasePath(path: string): string {
	return `/${path.split("/").filter(a => a)[0]}`;
}
