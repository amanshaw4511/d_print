# d_print
This library provide to easily print a struct in rust.

## Usage
### Using  ```DisplayPrint``` trait
```rust
x.print();
// equivalent to
print!("{}", x);

x.println();
// equivalent to 
println!("{}", x);
```
Here ```x``` must implement ```Display``` trait


### Using  ```DebugPrint``` trait
```rust
x.dprint();
// equivalent to
print!("{:?}", x);

x.dprintln();
// equivalent to 
println!("{:?}", x);
```
Here ```x``` must implement ```Debug``` trait

### Examples
```rust
use std::fmt::Display;
use d_print::{DisplayPrint, DebugPrint};

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}

fn main() {
    1.print();
    "hello".println();
    2.4.println();
    let origin = Point { x: 0, y: 0 };
    origin.print();
    origin.dprint();
}

// Output //
1hello
2.4
<0,0>
Point { x: 0, y: 0 }
////////////

```
