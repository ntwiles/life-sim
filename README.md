## Tools

### Graphviz

A sample of each generation has it's neural network outputted to .dot files in the `/data` directory. Images can be
generated from these files using [DOT command line](https://graphviz.org/download/) with the following example command:

```
dot -Tpng ./data/dots/0.dot -o ./data/graphs/0.png
```

### Pyplot

A python script in the `./scripts` folder can be used to chart survivorship over time. To use, pipe the simulator stdout
into the python plotting script:

```
cargo run | python scripts/plot.py
```
