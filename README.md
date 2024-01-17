* Lessons: [week 1](https://github.com/alfredodeza/rust-setup), [week 2](https://github.com/alfredodeza/rust-fundamentals), [week 3](https://github.com/alfredodeza/rust-structs-types-enums/), [week 4](https://github.com/alfredodeza/applied-rust)

# CLI

- `cat /etc/os-release`
- `free -m`
- `which cargo`
- `cargo init .` creates the application in the current directory using the name of the directory for the application's name.
- `tree`
- `cargo init --lib` creates a library instead of an application.

# Docs

- Teachers:
  - Alfredo Deza (https://www.coursera.org/instructor/~81359500, https://github.com/alfredodeza/alfredodeza)
  - Noah Gift (https://www.coursera.org/instructor/noahgift, https://github.com/noahgift, https://www.oreilly.com/pub/au/3039)
- https://github.com/features/copilot
- https://github.com/settings/copilot
- https://docs.github.com/en/copilot/quickstart
- CodeWhisperer as an alternative to Copilot:
  - https://aws.amazon.com/codewhisperer
  - https://marketplace.visualstudio.com/items?itemName=AmazonWebServices.aws-toolkit-vscode
- https://github.com/settings/billing/summary
- https://docs.github.com/en/billing/managing-billing-for-github-codespaces/about-billing-for-github-codespaces#monthly-included-storage-and-core-hours-for-personal-accounts
- https://techcommunity.microsoft.com/t5/educator-developer-blog/how-to-optimize-your-codespaces-pro-tips-for-managing-quotas/ba-p/3712032
- https://code.visualstudio.com/docs/languages/rust
- https://code.visualstudio.com/docs/editor/settings-sync
- https://learn.microsoft.com/en-us/training/modules/rust-set-up-environment 
- https://learn.microsoft.com/en-us/training/modules/rust-introduction
- https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each
- https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
- https://rust-book.cs.brown.edu/ch03-05-control-flow.html#using-if-in-a-let-statement
- https://learn.microsoft.com/en-us/training/paths/rust-first-steps
  - https://learn.microsoft.com/en-us/training/modules/rust-loop-expressions
  - https://learn.microsoft.com/en-us/training/modules/rust-if-else-expressions
  - https://learn.microsoft.com/en-us/training/modules/rust-create-program
  - https://learn.microsoft.com/en-us/training/modules/rust-memory-management
- https://github.com/alfredodeza/rust-template
- https://doc.rust-lang.org/rust-by-example/std_misc/arg.html
- https://github.com/alfredodeza/rust-fundamentals/blob/main/examples/16-error-handling/error-handling/src/main.rs
- https://doc.rust-lang.org/nightly/std/env/fn.args.html
- https://doc.rust-lang.org/nightly/std/io/struct.Error.html
- https://doc.rust-lang.org/nightly/std/io/struct.Error.html#method.new
- https://doc.rust-lang.org/nightly/std/io/enum.ErrorKind.html
- https://doc.rust-lang.org/nightly/std/fs/fn.read_to_string.html
- Use [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone) when you have a [`String`](https://doc.rust-lang.org/std/string/struct.String.html) and you want to make a copy of it.
- Use [`to_owned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned) when you have a [`&str`](https://doc.rust-lang.org/std/primitive.str.html) and you want to create an owned [`String`](https://doc.rust-lang.org/std/string/struct.String.html) from it.
- Use [`format!`](https://doc.rust-lang.org/std/macro.format.html) when you want to create a [`String`](https://doc.rust-lang.org/std/string/struct.String.html) with formatted text.
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html) is growable and mutable whereas [`&str`](https://doc.rust-lang.org/std/primitive.str.html) is not.
- [`String`](https://doc.rust-lang.org/std/string/struct.String.html) is owned by the code that creates it.
- https://learn.microsoft.com/en-us/training/modules/rust-loop-expressions
- Use [slices](https://doc.rust-lang.org/std/primitive.slice.html) when: 
  - you want to borrow a portion of a collection rather than the whole collection.
  - you want to pass around a reference to a sequence of items without copying them.
  - you want to access a subset of a collection without copying.
- Use [vectors](https://doc.rust-lang.org/std/vec/index.html) when:
  - you need to dynamically grow or shrink your collection.
  - you need to own the collection and transfer ownership to another scope.
- https://learn.microsoft.com/en-us/training/modules/rust-error-handling
- Advantages of using an enum (according to Bard):
  - *Easier to read and understand*: Enums are generally considered to be easier to read and understand than structs, especially for complex data structures. This is because enums use a more declarative style of programming, which can make code more concise and easier to follow.
  - *More flexible*: Enums can be more flexible than structs in that they can represent a wider range of values. For example, an enum could be used to represent not just byte sizes, but also other types of data, such as colors or shapes.
- Disadvantages of using an enum (according to Bard):
  - *Less efficient*: Enums can be slightly less efficient than structs in terms of memory usage and performance. This is because enums store additional information about the variant that is currently being used, which can take up some additional space in memory.
  - *Less type safety*: Enums are less type-safe than structs in that they do not enforce the same level of consistency between variants. This can make it more difficult to catch errors at compile time.



