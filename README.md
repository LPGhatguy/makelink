# makelink
You can't use `mklink` from Bash on Windows because it's a Batch built-in. The version of `ln` that comes with Git Bash is really broken.

Enter makelink. It makes symlinks.

## Installation
You need a recent build of Rust.

```sh
cargo install makelink
```

## Usage
```
makelink <link name> <link target>
```

Example:

```sh
echo "hi" > foo.txt
makelink bar.txt foo.txt
cat bar.txt # hi
```

If you forget the order of the arguments (you will), `mklink --help` is small and to-the-point:

```
USAGE:
    makelink.exe <LINK> <TARGET>

...

ARGS:
    <LINK>      Where the symlink should be created
    <TARGET>    Where the symlink should point
```

## License
makelink is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.