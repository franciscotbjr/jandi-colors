import { describe, it, expect } from 'vitest'
import { toRGBA, hexToRgb } from '../src/utils'
import { primary } from '../src/palette'

describe('toRGBA — input validation', () => {
  it('accepts valid alpha range', () => {
    expect(toRGBA(primary, 0)).toMatch(/^rgba\(/)
    expect(toRGBA(primary, 0.5)).toMatch(/^rgba\(/)
    expect(toRGBA(primary, 1)).toMatch(/^rgba\(/)
  })

  it.each([
    NaN,
    Infinity,
    -Infinity,
    -0.1,
    1.1,
    2,
    -1,
    '0' as any,
    '0); evil' as any,
    null as any,
    undefined as any,
  ])('rejects invalid alpha: %p', alpha => {
    expect(() => toRGBA(primary, alpha)).toThrow()
  })

  it('does not reflect alpha value in error message', () => {
    try {
      toRGBA(primary, 2)
      throw new Error('should have thrown')
    } catch (e: any) {
      expect(e.message).not.toContain('2')
    }
  })
})

describe('hexToRgb — adversarial inputs', () => {
  it('rejects injection attempts without reflecting full payload', () => {
    const payload = "'; DROP TABLE users; --"
    try {
      hexToRgb(payload)
      throw new Error('should have thrown')
    } catch (e: any) {
      expect(e.message).not.toContain(';')
      expect(e.message).not.toContain('<')
    }
  })

  it('rejects empty, oversized, and malformed input', () => {
    expect(() => hexToRgb('')).toThrow()
    expect(() => hexToRgb('###')).toThrow()
    expect(() => hexToRgb('a'.repeat(10_000))).toThrow()
    expect(() => hexToRgb('ZZZZZZ')).toThrow()
  })
})
