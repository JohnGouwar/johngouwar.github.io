---
id: niszk-l 
order: 1
title: > 
    Cryptographic Hardness under Projections for Time-Bounded Kolmogorov 
    Complexity
year: 2023
authors: ["Eric Allender", "John Gouwar", "Shuichi Hirahara", "Caleb Robelle"]
venue: Theoretical Computer Science
doi: https://doi.org/10.1016/j.tcs.2022.10.040
abstr: niszkl.md 
---
A version of time-bounded Kolmogorov complexity, denoted `KT`, has received
attention in the past several years, due to its close connection to circuit
complexity and to the Minimum Circuit Size Problem `MCSP`. Essentially all
results about the complexity of `MCSP` hold also for `MKTP` (the problem of
computing the complexity of a string). Both and are hard for `SZK` (Statistical
Zero Knowledge) under `BPP`-Turing reductions; neither is known to be
`NP`-complete.

Recently, some hardness results for `MKTP` were proved that are not (yet) known
to hold for `MCSP`. In particular, is hard for `DET` (a subclass of `P`) under
nonuniform reductions. In this paper, we improve this, to show that co-`MKTP` is
hard for the (apparently larger) class `NISZKL` under not only `NC0` many-one
reductions but even under projections. Also co-`MKTP` is hard for `NISZK` under
non-uniform polynomial many-one reductions. Here, `NISZK` is the class of
problems with non-interactive zero-knowledge proofs, and `NISZKL` is the
non-interactive version of the class `SZKL` that was studied by Dvir et al.

As an application, we provide several improved worst-case to average-case
reductions to problems in `NP`, and we obtain a new lower bound on `MKTP` (which
is currently not known to hold for `MCSP`).