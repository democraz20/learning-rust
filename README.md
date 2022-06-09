# learning-rust
learning rust </br>
project are not in order rather than in commit history </br>

testing read/write to file and basic math
______________________________________________________________

```rs
18 |    let mut file = File::create("file.txt").unwrap();
```
here File::create does not have the ``read`` to be true
```rs
31 |    let mut file = File::open("file.txt").unwrap();
```
recreate the var with File::open so ``read`` is true </br>
now reading file contents to var is possible </br>
______________________________________________________________

didn't use ``OpenOptions::new();`` because it returns </br>
``file = Ok (..)`` instead, when using ``File::``</br>
it returns ``&file = File {..}`` which is what open and write requires
all this using ``dbg!();``
______________________________________________________________