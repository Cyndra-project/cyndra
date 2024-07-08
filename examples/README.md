# Examples

Some example apps to show what you can do with cyndra.

## How to deploy the examples

To deploy the examples, check out the repository locally

```bash
$ git clone https://github.com/getsynth/cyndra.git
```

navigate to an example root folder

```bash
$ cd examples/rocket/hello-world
```

open up the `Cyndra.toml` file and change the project name to something 
unique - in cyndra, projects are globally unique. Then run

```bash
$ cargo cyndra deploy
```