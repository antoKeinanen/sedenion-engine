import test from 'ava'
import { evaluate } from '../../index.js'

test('can add two numbers', (t) => {
    t.is(evaluate("2+2"), 4)
})

test("can add floating point numbers", (t) => {
    t.is(evaluate("2.5+2.5", 5))
});
