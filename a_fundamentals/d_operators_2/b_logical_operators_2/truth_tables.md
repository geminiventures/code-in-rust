# Truth Tables
Logical operators have a special table called \"Truth table\" that shows what the combination of logical operators returns.

Truth table for the `and` (`&&`) operator:

| a     | b     | a && b |
| ----- | ----- | ------ |
| false | false | false  |
| false | true  | false  |
| true  | false | false  |
| true  | true  | true   |

The only way to get a `true` for the `and` (`&&`) operator is if both `a` and `b` are `true`

Truth table for the `or` (`||`) operator:

|a|b|a \| b|
|---|---|---|
|false|false|false|
|false|true|true|
|true|false|true|
|true|true|true|

In this case, to get a `true` result, either `a` or `b` should be `true`.

Truth table for the `not` (`!`) operator:

|a|!a|
|---|---|
|false|true|
|true|false|

Here the value of `a` is reversed. If `a` is `false` then `!a` is `true`

![quiz](https://coddy.tech/icons/quiz-main.svg)

Navigate to quiz

![](https://coddy.tech/icons/right-main.svg)

![](https://coddy.tech/icons/lesson-white.svg)

![](https://coddy.tech/icons/quiz-white.svg)

Logical Operators Part 1 | Rust