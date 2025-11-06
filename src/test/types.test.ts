import { describe, expect, it } from 'vitest';
import { isApiError } from '../lib/types';

describe('Type Guards', () => {
  it('should identify API errors correctly', () => {
    const error = { message: 'Test error' };
    expect(isApiError(error)).toBe(true);

    const notError = { id: 1, name: 'test' };
    expect(isApiError(notError)).toBe(false);

    expect(isApiError(null)).toBeFalsy();
    expect(isApiError(undefined)).toBeFalsy();
  });
});
