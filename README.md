# Bitflip Scanner

Have you ever wondered how often your memory is silently corrupted by cosmic rays, terrestrial radiation, or boring old heat? Now you can find out!

This program allocates a large chunk of memory and populates it with predictable values according to some function of the array index. Then the program scans the memory at regular intervals to ensure that the value at the expected location is still the value we expect it to be. If it's not, we have a bitflip event!

In order for this to work reliably, you should run it on a mostly-idle machine with enough free RAM to ensure that this program will not be swapped out to disk. If the pages of memory get swapped, they're re-written to memory when we need to scan again thus erasing any evidence of a bitflip if one occurred. 

You can compile and run the program with Cargo like so:

```bash
cargo run --release -- 2 >> bitflip_log.txt
```

The single parameter corresponds to how many Gibibytes of memory we allocated for scanning. I refer to this as "scale" because the amount of memory allocated directly determines the physical size of our "detector". A larger detector will detect events more often than a smaller one. All that said, a given amount of memory on one machine may not be physically the same size as that on another machine, so "scale" represents a relationship of proportions and not exact measurements. 