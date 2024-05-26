import { PUBLIC_API_URL } from "$env/static/public";

/**
 * Send a GET request to the API.
 * 
 * @param uri The URI to fetch.
 */
export async function get(uri: string) {
	try {
		const res = await fetch(`${PUBLIC_API_URL}${uri}`, {
			headers: {
				'Authorization': await resolveAuthHeader(),
				'Accept': 'application/json'
			}
		});
		return await res.json();

	} catch (e) {
		console.log('GET request failed:', e);
		return Promise.reject();
	}
}

/**
 * Send a POST request to the API.
 * 
 * @param uri The URI to POST.
 * @param body The body of the request - should be stringified JSON.
 */
export async function post(uri: string, body: string) {
	const res = await fetch(`${PUBLIC_API_URL}${uri}`, {
		method: 'POST',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		},
		body
	});

	if (res.status !== 200) {
		throw new Error("Failed to perform POST request");
	}
}


/**
 * Send a PUT request to the API.
 * 
 * @param uri The URI to POST.
 * @param body The body of the request - should be stringified JSON.
 */
export async function put(uri: string, body: string) {
	return await fetch(`${PUBLIC_API_URL}${uri}`, {
		method: 'PUT',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json',
			'Content-Type': 'application/json',
		},
		body
	});
}

/**
 * Send a DELETE request to the API.
 * 
 * @param uri The resource to delete.
 */
export async function del(uri: string) {
	const res = await fetch(`${PUBLIC_API_URL}${uri}`, {
		method: 'DELETE',
		headers: {
			'Authorization': await resolveAuthHeader(),
			'Accept': 'application/json'
		}
	});

	if (res.status !== 204) {
		throw new Error("Failed to perform DELETE request");
	}
}

/** Resolve the authorisation header to use for the current request (placeholder) */
export async function resolveAuthHeader(): Promise<string> {
	return "1";
}