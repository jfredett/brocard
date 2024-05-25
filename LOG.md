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

# 25-MAY-2024

## 0012

Alright, I've got a brocard benchmark in place, I'm slow as molasses. I think the next step is
profiling, I don't want to jump to a SIMD conversion until I'm sure I've got the algorithm right.

Another thing that occurs to me to check is the generated code to ensure there aren't any division
instructions being written. Ultimately, the point of montgomery arithmetic is to dodge divisions by
replacing them with shifts; so I would assume an audit of the generated code to verify there are
only divisions where I think there should be divisions makes sense.

I'm getting this committed, then I'm going to read
[this](https://towardsdatascience.com/nine-rules-for-simd-acceleration-of-your-rust-code-part-1-c16fe639ce21),
which seems like an interesting article. Right now, I think there are two paths forward with SIMD.

First, I could rewrite the existing implementation to parameterize it's `Elt` type, instead of u128,
it's be T. I have no idea what effect that'll have on performance, and I imagine it will deeply
change how much I have to appease the trait system, but I think it's a theoretical `K`-times speedup for
essentially a bunch of gruntwork when I sub in `u64xK` or whatever it is. The downside is this
cannot exceed some value slightly below 2^64, since I'll need primes larger than `n`, and those
primes have to be represented as a 64b number to make good use of SIMD in this model. I don't see
any 128b types allowed in the docs I've looked at, but I haven't looked particularly hard since
`signac-vector`.

Second, I could focus on something past the `2^64 (~1e19)` barrier. While it is 4 orders of
magnitude ahead, and it took months to go from 1e12 to 1e15, I am approaching this with a scaling
game in mind; I'm honestly not all that interested in Brocard itself, I'm interested in building out
the thing that scales the solution and making that efficient, so it may be wise to focus on the
multiprecision brocard implementation. That should be easy to make SIMD/vectorizing friendly, to the
point where I would expect most of the relevant loops to be vectorized by the compiler. A naive
(not explicitly SIMDified) implementation would obviate the barrier and also potentially be
sufficiently quick (with the added horsepower I have) to not hinder progress.

Multiprecision is also the way to go for GPU implementation, which is still somewhere in the cards.

In any case, the final thing I need to do is some cleanup in the repo, get things organized, tests
moved to their rightful place, some documentation work, and hopefully by the time I'm done I'll feel
like figuring out performance tracing for what feels like the thousandth time.


## 0038

Reading this article was a very good idea, SIMD does not work the way I thought it did anymore.

## 0052

Finished reading (well, skimming through) the article. I think I know the approach, the answer is
'both'. I can optimize the existing algorithm using this new `core::simd` stuff mentioned in the
article to get familiar with it. This will allow me to hit `2^61 - 1` as my largest prime, with
`R_EXP=61`. That's about 1e18, or 3 orders of magnitude from the previous attempt.

Next, I'll implement a multiprecision montgomery version using SIMD while the initial version is
running. The algorithm with generate Brocard Spans and dump all the relevant metadata to disk.
Periodically they will run the same span to verify they are both producing the same result. I think
the managing apparatus will just be a couple systemd-wrapping-sinatra or something to drive them and
report results to some parent process I can run on toto.


