# kukumba

[BDD helpa](https://www.youtube.com/watch?v=IkhDKURfks4).
Makes test cases more readable.

`TODO` output just the spec

## Usage

```rust
#[macro_use] extern crate kukumba;

kukumba!(
  #[scenario_01]

  given "something"
  and "nothing else" {
    let something = "cucumba";
  }

  when "something happens" {
    let slice = &cucumba[3..];
  }
  and "nothing else happens" {
    // ...
  }

  then "only that should've happened" {
    assert_eq!(slice, &"umba");
  }
  and "everything should be okay"
  and "everyone should be happy" {
    assert_eq!(something, "cucumba");
  }

  #[scenario_02]
  // ...
)
```
