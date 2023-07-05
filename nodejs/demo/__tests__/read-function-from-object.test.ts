import { expect, describe, test } from 'vitest'
import * as wasm from '../wasm/read_function_from_object'

describe('read-function-from-object', () => {
  describe('using js_sys', () => {
    test('it works!', () => {
      const stack = [] as string[]
      const obj = {
        fn1: () => {
          stack.push('fn1')
        },
        fn2: () => {
          stack.push('fn2')
        },
      }
  
      const jsExecutor = new wasm.JsExecutor(obj)
      jsExecutor.call_fn1()
      jsExecutor.call_fn2()
      jsExecutor.call_fn2()
      jsExecutor.call_fn1()
  
      expect(stack).toEqual(['fn1', 'fn2', 'fn2', 'fn1'])
    })
  
    test('it panics when fn1 is not found', async () => {
      const obj = {
        fn2: () => {},
      }
  
      try {
        new wasm.JsExecutor(obj)
      } catch (e) {
        const error = e as Error
        expect(error instanceof Error).toBeTruthy()
        expect(error.name).toEqual('RuntimeError')
        expect(error.message).toEqual('unreachable') // panic!
      }
    })
  
    test('it panics when fn2 is not found', async () => {
      const obj = {
        fn2: () => {},
      }
  
      try {
        new wasm.JsExecutor(obj)
      } catch (e) {
        const error = e as Error
        expect(error instanceof Error).toBeTruthy()
        expect(error.name).toEqual('RuntimeError')
        expect(error.message).toEqual('unreachable') // panic!
      }
    })
  })

  describe('using getter_with_clone + ducktor', () => {
    test('it works!', () => {
      const stack = [] as string[]
      const obj = {
        fn3: () => {
          stack.push('fn3')
        }
      }
  
      // @ts-ignore
      wasm.call_fn3(obj)
      wasm.call_fn3(obj)
      wasm.call_fn3(obj)
  
      expect(stack).toEqual(['fn3', 'fn3', 'fn3'])
    })
  })

  test('it throws when fn3 is not found', async () => {
    const obj = {}

    try {
      wasm.call_fn3(obj)
    } catch (e) {
      const error = e as Error
      expect(error instanceof Error).toBeTruthy()
      expect(error.name).toEqual('Error')
      expect(error.message).toEqual('Unknown error') // error!
    }
  })
})
