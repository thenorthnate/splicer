# splicer
Tool for working with splicing data

## Approach
1. Create a reference index for the exon's listed in a reference annotation file. This is comprised of 2 parts - a vector that contains a sorted list of all the exons, and a "connection graph" where the nodes in the graph are exons, and an edge between node 1 and node 2 indicates that those two exons overlap in the genome.
2. Perform a time-series analysis of the BAM files by loading a maximum of 2 non-overlapping segments at a time and detecting read changes.
   1. Load all the reads from the first segment
