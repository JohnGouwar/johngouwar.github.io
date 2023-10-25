---
id: mapimgs
order: 0
title : >
  Deformable Part Models for Automatically Georeferencing Historical Map Images
year: 2019
authors: [
    "Nicholas R. Howe", 
    "Jerod Weinman", 
    "John Gouwar", 
    "Aabid Shamji"
]
venue: SIGSPATIAL
doi: https://doi.org/10.1145/3347146.3359367
---
Libraries are digitizing their collections of maps from all eras, generating increasingly large online collections of historical cartographic resources. Aligning such maps to a modern geographic coordinate system greatly increases their utility. This work presents a method for such automatic georeferencing, matching raster image content to GIS vector coordinate data. Given an approximate initial alignment that has already been projected from a spherical geographic coordinate system to a Cartesian map coordinate system, a probabilistic shape-matching scheme determines an optimized match between the GIS contours and ink in the binarized map image. Using an evaluation set of 20 historical maps from states and regions of the U.S., the method reduces average alignment RMSE by 12%.