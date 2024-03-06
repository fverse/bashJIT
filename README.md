# bashjit
Experimental Just In Time Compiler for bash

#### Note: This is a highly experimental project. Not guaranteed to be useful in present or/and future!

## Motivation

Bash scripts are notoriously slow. I don't fully understand why it is so, but one potential performance improvement could be JIT compilation instead of interpreting. And I can't see any existing JIT compilers for Bash (Maybe this is because developing one is infeasible). However, implementing JIT compilation won't be feasible for the entire language. Instead, we could aim for optimizing common bottlenecks (loops, conditionals, etc.). To do this, what I am planning is start with a limited subset of Bash syntax to get an early prototype working. Expand this incrementally as far as possible. That being said, in some cases, implementing a JIT compiler for Bash might not offer significant (or any) performance gains, so this remains as highly experimental.

## Bash Script Performance 

#### Loops 
Following is a `for` loop in Bash: 

```bash
#!/bin/bash

x=0

for (( i=0; i<1000000; i++ )); do
    x=$((x + 1))
done
```

It takes 3.366 seconds to execute on my machine. The same `for` loop in JavaScript and Python only takes 0.062 and 0.139 seconds respectively. That means ~50x slower than JavaScript and ~20x slower than Python  

#### Variable name length

Unlike JavaScript, Python or other languages, variable names significantly affect Bash performance. Following is the same `for` loop with longer variable names. But it took 4.870 seconds to complete. Compared to 3.366 seconds for the shorter version, it's ~40% slower. It's expected the variable length to have some impact on Bash runtime since it is interpreted, but this much  slowdown is unexpected.

```bash
#!/bin/bash

# Using long variable names
veryLongVariableNameForCounter=0

for (( indexInLongRunningLoop=0; indexInLongRunningLoop<1000000; indexInLongRunningLoop++ )); do
    veryLongVariableNameForCounter=$((veryLongVariableNameForCounter + 1))
done
```

#### More of this

There are other similar scenarios. I will address them once I get there.

