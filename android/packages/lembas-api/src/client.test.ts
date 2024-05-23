import { describe, expect, it, jest } from '@jest/globals';
import { resolveAuthHeader } from './client';

const mockAuthSession = {
	tokens: {
		accessToken: 'token'
	}
};

jest.mock('aws-amplify/auth', () => {
	return {
		fetchAuthSession: jest.fn(async () => Promise.resolve(mockAuthSession))
	};
});

describe('resolveAuthHeader', () => {
	it('should return a bearer token from the current auth session', async () => {
		const header = await resolveAuthHeader();
		expect(header).toEqual('Bearer token');
	});
});