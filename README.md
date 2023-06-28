# Description

A game, or demo, about a robin and some fish. Written in `rust` using `bevy` targeting native and web deployment. 

## Running

Whether you just want the binary or want to develop through the docker container, run;

```
docker run -v "$PWD":/usr/src/simple_collision simple_collision[_targets]
```
## Building
### Targets

```
sudo docker build -t simple_collision_targets --target build .
```

If you're only interested in running the project, target the build stage then run as described [above](#running), this will update your `target` dir with the newly built debug binary.

You can pass different commands to `docker run` to build different binary targets. See the [`cargo build` documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html) for options.

### Development

!! TODO: dev environment doesn't actually work because I need to figure out how to allow a docker container to spawn a window in the host wm. You can use the image to build but will need to run the binary on your own. 

```
sudo docker build -t simple_collision .
```

Currently the docker image is specifically intended for development use. 

The last build stage will install `cargo-watch` to run `cargo run` each time you make a change in the project's `src/` dir. This helps cut down on build time by persisting cargo's cache. 
