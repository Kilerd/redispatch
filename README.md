<div align="center">
    <h1>redispatch</h1>
    <p>best serialization patch for redis</p>
    <img alt="GitHub Workflow Status (with branch)" src="https://img.shields.io/github/actions/workflow/status/kilerd/redispatch/ci-for-main.yaml?branch=main">
    <a href="https://crates.io/crates/redispatch"><img alt="Crates.io" src="https://img.shields.io/crates/v/redispatch"></a>
    <a href="https://codecov.io/gh/kilerd/redispatch" >
    <img src="https://codecov.io/gh/kilerd/redispatch/branch/main/graph/badge.svg"/>
    </a>
    <a href="https://crates.io/crates/redispatch">
    <img alt="Crates.io (recent)" src="https://img.shields.io/crates/dr/redispatch"></a>
    <a href="https://docs.rs/redispatch"><img alt="docs.rs" src="https://img.shields.io/docsrs/redispatch"></a>
    <img alt="Crates.io" src="https://img.shields.io/crates/l/redispatch">
</div>

## Features
 - work fine with redis lib

## Installation
```sh
$ cargo add redispatch
```

## Example
```rust
use futures::prelude::*;
use redispatch::JsonSerdeCommands;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, DeserializeOwned)]
struct MyData {
    username: String
}

let client = redis::Client::open("redis://127.0.0.1/").unwrap();
let mut con = client.get_async_connection().await?;

let serde_ret = con.get::<MyData>("mydata").await?;
```

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:
- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]


## License
This project is licensed under either of the following licenses, at your option:
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])


[contributing]: https://github.com/kilerd/redispatch/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/kilerd/redispatch/labels/good%20first%20issue
[help-wanted]: https://github.com/kilerd/redispatch/labels/help%20wanted