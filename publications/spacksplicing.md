---
id: spack-splicing
order: 5
title: >
    Bridging the Gap Between Binary and Source Based Package Management in Spack
year: 2025
authors: [
  "John Gouwar",
  "Gregory Becker",
  "Tamara Dahlgren", 
  "Nathan Hanford", 
  "Arjun Guha",
  "Todd Gamblin"
]
venue: Super Computing 
arxiv: https://arxiv.org/abs/2509.07728
---
Binary package managers install software quickly but they limit configurability
due to rigid ABI requirements that ensure compatibility between binaries. Source
package managers provide flexibility in building software, but compilation can
be slow. For example, installing an HPC code with a new MPI implementation may
result in a full rebuild. Spack, a widely deployed, HPC-focused package manager,
can use source and pre-compiled binaries, but lacks a binary compatibility
model, so it cannot mix binaries not built together. We present splicing, an
extension to Spack that models binary compatibility between packages and allows
seamless mixing of source and binary distributions. Splicing augments Spack's
packaging language and dependency resolution engine to reuse compatible binaries
but maintains the flexibility of source builds. It incurs minimal
installation-time overhead and allows rapid installation from binaries, even for
ABI-sensitive dependencies like MPI that would otherwise require many rebuilds.
