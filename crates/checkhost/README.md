# checkhost

check-host.net api wrapper as a library.

(Functions documentation not yet set)

## Usage

```rs
use checkhost;
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let check_result = checkhost::check_http(host, nodes)?;
    println("{:?}", check_result);
}
```
