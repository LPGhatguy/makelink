# makelink
`mklink` isn't usable from non-cmd.exe shells on Windows because it's a Batch built-in.

The version of `ln` that comes with Git Bash on Windows copies files instead of creating actual symlinks.

Enter makelink. It makes symlinks. It runs on Windows, MacOS, Linux, and maybe elsewhere.

## Installation
You need a recent build of Rust.

```sh
cargo install makelink
```

## Usage
It's the same as the `cp` command.

```
makelink <source> <dest>
```

Example:

```sh
echo "hi" > foo.txt
makelink foo.txt bar.txt
cat bar.txt # hi
```

If you forget the order of the arguments (you will), `makelink --help` is small and to-the-point:

```
USAGE:
    makelink.exe <source> <dest>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <source>    This is where the link will point to
    <dest>      This is where the link will be created
```

## License
makelink is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.
