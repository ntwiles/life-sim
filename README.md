# Life Simulator

This simulates biological life and evolution in cellular automata. Each entity has a randomly generated and sparsely
connected neural network, which it uses to make decisions on how best to survive a selection scenario. At the end of each
generation, each surviving entity passes is genome on to a number children. The genome of each child has a chance to
randomly mutate, before it's used to generate a new neural network.

This uses a [cellular autamata framework](https://github.com/ntwiles/cellular-automata) to handle rendering. The simulation
can be paused or resumed using the [space] key, and rendering can be (for better performance) enabled or disabled with
the [r] key.

## Tools

### Graphviz

A sample of each generation has it's neural network outputted to .dot files in the `/data` directory. Images can be
generated from these files using [DOT command line](https://graphviz.org/download/) with the following example command:

```sh
dot -Tpng ./data/dots/0.dot -o ./data/graphs/0.png
```

### Pyplot

A python script in the `./scripts` folder can be used to chart survivorship over time. To use, pipe the simulator stdout
into the python plotting script:

```sh
cargo run | python scripts/plot.py
```
