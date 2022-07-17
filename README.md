## Raft

- goal is providing
  a complete and practical foundation for system building

### two techniques (generally applicable)

1) problem decomposition:
    - wherever possible, we divided problems into separate pieces
      that could be:
        - solved
        - explained
        - understood _relatively independently_
    - EX: We separated leader election, log replication, safety, and membership changes.
2) simplify the state space:
    - reducing the number of states to consider
    - make the system more coherent
    - eliminating nondeterminism where possible
    - EX: Logs are not allowed to have
      holes, and Raft limits the ways in which logs can become
      inconsistent with each other
