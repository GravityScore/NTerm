
# NTerm

A high level ncurses wrapper for Rust, which makes writing terminal applications with a pseudo-GUI much easier (raw ncurses is a nightmare).

## Documentation

Found [here](https://gravityscore.github.io/NTerm/nterm/index.html).

## Usage

The version of ncurses installed by default on OSX doesn't have all the necessary features for nterm to compile properly (you'll get a scary linker error). So if you're on OSX, go ahead and download the latest ncurses uing Homebrew:

```bash
$ brew install --prefix=/usr/local --force ncurses
```

Now you need to tell the Rust compiler to look inside `/usr/local` when searching for libraries. Add `/usr/local` to the environment variable `LD_LIBRARY_PATHS`:

```bash
$ export LD_LIBRARY_PATHS="$LD_LIBRARY_PATHS:/usr/local/lib
```

Now add nterm as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
nterm = "*"
```
