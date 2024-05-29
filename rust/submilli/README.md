# submilli

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v0.8.1. [Bun](https://bun.sh) is a fast all-in-one JavaScript runtime.

# Deterministic simulation

## The simulator will amplify randomness, create chaos and inject failures into your system intendedly.

1.  a lot of hidden bugs may be revealed, which you can then **deterministically reproduce** them until they are fixed.
2.  if your system is survive such chaos, you will have more confidence in results.

## What we need:

1. all I/O-related interfaces must be mocked during the simulation.
2. all uncertainties should be eliminated.
