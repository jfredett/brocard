# 21-MAY-2024

## 1555

Current status: Hauling ass.

I set up a brocard run up to 1e9, as an initial test. It's currently quite a bit slower than the jhg
version, but I think for an unoptimized bit of hackery it's doing alright. `rayon` is handling
parallelizing, but it's the stupidest implementation I could come up with so I think there's more to
gain by just improving the code a bit.

As mentioned above, there's an outer loop that's causing a lot of slowness on the inner loop by
recalculating factorial over and over. I'm also not being particularly efficient with how I break up
work. I should be controlling the work-unit size and assigning threads to a smaller sub-span of
whatever input span I get.

I can then precompute all the relevant montgomery spaces, and share some of the work of calculating
the factorial of all those spaces.

I suppose since each number is only 16B wide, if I plan to precompute all the factorials for a given
range, then the total memory allocation is only `16*(UPPER_BOUND - START)`. If I split off the
parent range into 'sections', I can probably manage the memory use and then simply precompute the
factorial for the whole range, then iterate over each to do the series of legendre tests,
essentially turning the loop inside out and saving all that time.

I need to set up proper benchmarks for that, and probably learn a bit more about Rayon.

I also need to look into SIMD optimization for this, but it's limited inasmuch as I want to use 128b
values in most places, but nothing appears to support lanes >64b.

I want to push the boundary to arbitrary heights, but I suppose refactoring the existing impl to at
least by type-generic might be worth it if it doesn't hurt performance. It may be faster to cover
all 64b numbers first, then move to a multi-precision model which will be slower but can proceed
unbounded.


