# pipfloat - Make your pip... float

A terrible spaghetti code that checks if a windows title is "Picture-in-Picture" and makes it float if it isn't already floating.

The code is pretty bad, there is no error checking at all, it's using an uncomplete and unofficial library, there is no logging of any kind and overall its very much not flexible.

I will not put any more effort than this since it'd be better spent in making hyprland have support for dynamic title out of the box.

### Usage

Download the latest release [here](https://github.com/morr0ne/pipfloat/releases/latest/download/pipfloat), simply copy it to your PATH and add it to your hyprland config like so:
```
exec-once = pipfloat
```

### Compiling

If for some reason you want to compile it yourself simply install the rust compiler, clone the repo and run

```bash
cargo build --release
```

The resulting binary can be found at `target/release/pipfloat`
