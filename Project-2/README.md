# Todo:
* Create archinve for storing fitness values.
* Parallel computing
* MPI

# Too time consuming
* Use surrogate model with ML.

# Own ideas
* Heuristic mutation (swap those who are closer, or based on start_time/end_time)

# SSH
tmux new -s bio-rust

cargo run --release

crtl+B, then D

Attach later:
tmux attach -t bio-rust