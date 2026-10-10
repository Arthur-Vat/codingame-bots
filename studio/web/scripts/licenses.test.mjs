import { describe, expect, it } from 'vitest';
import { isLicenseAllowed } from './licenses.mjs';

const allowed = ['MIT', 'Apache-2.0', 'ISC'];

describe('isLicenseAllowed', () => {
  it('accepts an allowed license', () => {
    expect(isLicenseAllowed('MIT', allowed)).toBe(true);
  });

  it('refuses a license that is not allowed', () => {
    expect(isLicenseAllowed('GPL-3.0', allowed)).toBe(false);
  });

  it('accepts an OR when one side is allowed', () => {
    expect(isLicenseAllowed('(MIT OR GPL-3.0)', allowed)).toBe(true);
    expect(isLicenseAllowed('GPL-3.0 OR MIT', allowed)).toBe(true);
  });

  it('refuses an OR when no side is allowed', () => {
    expect(isLicenseAllowed('(GPL-3.0 OR LGPL-2.1)', allowed)).toBe(false);
  });

  it('refuses an AND when one side is not allowed', () => {
    expect(isLicenseAllowed('MIT AND GPL-3.0', allowed)).toBe(false);
  });

  it('accepts an AND when both sides are allowed', () => {
    expect(isLicenseAllowed('(MIT AND ISC)', allowed)).toBe(true);
  });

  it('binds AND tighter than OR and honours parentheses', () => {
    expect(isLicenseAllowed('MIT OR GPL-3.0 AND GPL-2.0', allowed)).toBe(true);
    expect(isLicenseAllowed('(MIT OR GPL-3.0) AND GPL-2.0', allowed)).toBe(false);
  });

  it('refuses a missing, empty or malformed license', () => {
    expect(isLicenseAllowed(undefined, allowed)).toBe(false);
    expect(isLicenseAllowed('', allowed)).toBe(false);
    expect(isLicenseAllowed({ type: 'MIT' }, allowed)).toBe(false);
    expect(isLicenseAllowed('(MIT', allowed)).toBe(false);
    expect(isLicenseAllowed('MIT OR', allowed)).toBe(false);
    expect(isLicenseAllowed('MIT ISC', allowed)).toBe(false);
  });

  it('refuses an exception, which is not on the list', () => {
    expect(isLicenseAllowed('MIT WITH Classpath-exception-2.0', allowed)).toBe(false);
  });
});
