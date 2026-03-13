# Match Statement

The match expression allows you to compare a value against a series of patterns and execute code based on which pattern matches.

Here's the basic structure of a match expression:

```rust
match variable {
    pattern1 => expression1,
    pattern2 => expression2,
    // ... more patterns
    _ => default_expression,
}
```

The match keyword is followed by the value you want to test.

Each arm of the match consists of a pattern followed by `=>` and the code to execute.

The underscore _ is the default case that matches anything not matched by other patterns.

Here's an example:

```rust
let day = 3;
let day_name = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    // ... more patterns,
    _ => "Invalid day",
};
```

For multiple lines of code in an arm, use a block:

```rust
let day = 3;
let day_name = match day {
    1 => {
        println!("First day of the week!");
        "Monday"
    },
    2 => "Tuesday",
    // ... other cases
    _ => "Invalid day",
};
```

You can match multiple patterns using |:

```rust
let day = 3;
let day_type = match day {
    1 | 2 | 3 | 4 | 5 => "Weekday",
    6 | 7 => "Weekend",
    _ => "Invalid day",
};
```

Match expressions in Rust must be exhaustive, meaning they must cover all possible values. The compiler will check this for you.