# Rust Quadratic Solver

![jira-gif](./jira-cli.gif)

## Usage

```
rust-quadratic-solver 0.1.0
Created by Colin Huber
Quadratic Equation Solver ax^2 + bx + c
Example: rust-quadratic-solver.exe -- -a 1 -b -2 -c -3 -f output.csv

USAGE:
    rust-quadratic-solver.exe -a <a> -b <b> -c <c> --file <f>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a <a>
    -b <b>
    -c <c>
    -f, --file <f>    Output file
```

## Output

### Real Roots

If roots are real, the format is output as follows:

```csv
v1,r,<root1>,<root2>
```

Example:

```csv
v1,r,-2.0000,-3.0000
```

### Complex Roots

If the roots are imaginary, the format is output as follows:

```csv
v1,c,<root1real>+<complex1>i,<root2real>-<complex2>i
```

Example:

```csv
v1,c,-0.5000+0.8660i,-0.5000-0.8660i
```
