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


# 26-MAY-2024

## 1223

I have two big problems:

1. Get an ASM dump of the montgomery math to verify there are no divisions.
2. SIMDify montgomery math.

For #2, I think the actual algorithm isn't really SIMD friendly until I'm doing MP, but some of the
math functions might need some attention. I'm going to look over the C++ code from the prior work to
see if I can glean how they approached it, but my suspicion is that the SIMD will really come in on
the brocard side of the problem, not the montgomery side.

I could potentially SIMD to do some calculations in parallel, but I don't think it'll gain much.

The ASM dump to verify no divisions is a bit more straightforward. The #new code definitely has at
least a couple divisions by `r`, but I should be able to replace them w/ &'s. There's a `% n` in
there as well but I don't think that's going anywhere.

I replaced the `r` divisions and kicked off the benchmark, I expect little gain.

I'm going to refactor the montgomery math to use const generics for R_EXP, and then refactor the
benchmark to try every relevant R_EXP. The `redc` algorithm currently has to recalculate `r-1`
every time, and a const R_EXP would prevent that. It might also be worth genericizing over `u128` to
see if a u64 is faster.

## 1256

One way I might approach SIMDing the montgomery stuff is SIMDing the `.enter` side, and having a
single `n` and constant `R` for the space. So that a single Space can handle, e.g., `Cache -
Overhead` values.

## 1424

Refactoring to a const R_EXP appears to have improved things slightly, at least on Legendre symbol
benchmarking. Many of the benchmarks don't scale nicely to const generics so it's a little tricky to
tell. Next step is to refactor out the u128, but that is going to be a massive effort since every
math function relies on u128 and will need to be tweaked.

Refactoring that should transparently support SIMD-lanes, which would probably improve the existing
BrocardSpan implementation.

Overally there is a ~1-2% improvement from the R_EXP refactor, which is pretty good for such a small
change. Avoiding the extra calculations on each `redc` should pay off significantly on the brocard
benchmark. I'll run that next to see how it goes. I would expect to reap significant benefit since
`redc` is run on each multiply, and factorial necessarily multiplies `n` times, so a `1%` move on
`n` multiplies is a `1.01^n` move on the total time.

I can also consider refactoring _just_ the factorial function to SIMD to see if that adds an extra
bit of speed to brocard.

I also need to refactor TestCase to have a const R_EXP, and then do some more cleanup.

I did finish re-organizing the code, the tests are all still clumped in the `montgomery/mod.rs`
file, I'll probably pull them out to the `tests` directory since they're a little tough to organize
otherwise.

## 1440

I can't actually refactor the factorial because I can't use u128 w/ simd. ugh.

## 1809

Slight _regression_ for brocard, not sure why, really need to look at the ASM output. I suppose it
might be down to the fact that I make a bunch of these `Space` objects and the cost is in doing some
of the same precalculations over and over? Not sure.

Next step is to Genericize, I think, that'll at least allow me to process a small range of `n`'s
simultaneously, if I set it to some generic LANE size I can do some benchmarking to see what the
best size is, but I can relatively cheaply calculate a vector of `n_i!` over some small range `i` in
`[k, k+LANES]` since each `n_i = (n_i-1)i` which means I calculate `n_k!` and then each subsequent
element in the range is easily constructed in `LANES` operations, then I can legendre test all of
them; since each will 'terminate' at different times I can just have a `LANES`-sized mask that I
check to determine when to stop, meaning the Legendre test will take a worst-case time of the
slowest test in the range. I can also short circuit if I get a Non-witness report, since it doesn't
matter how many it passes, only that it fails on at least one.

## 2305

I did some work w/ `flamegraph` to chase out some remaining div_mod calls, that worked quite well.

I also found that the fairly naive splattering of `rayon` primitives around the solve function was
probably resulting in some deadlock or something, because it simply made my machine hot for 30m
while the sequential version runs to 1 billion in ~11m or so. Which, if I were to parallelize it
perfectly efficiently would mean I'm right in the same ballpark as the prior solution in terms of
speed.

One optimization I made that wasn't simply avoiding div_rem instructions was to avoid a `.exit()`
call in the Legendre symbol calculation. Since `0` is the same value for every `R`, we can just
check for `0` without leaving montgomery form, and it turns out that _entering_ montgomery form is
faster (despite having more steps) because there are only shifts and multiplies due to `redc`, so
you can minimize the math by comparing the result to see if it's equal to `space(1)`, rather than
`exit`ing as I was.


# 30-MAY-2024

## 2042

Okay, I did a bunch of work to start implementing the parallelized version of this. Yes, I am
putting off learning the SIMD stuff. I'm trying to sort out the right way to vectorize it, and I
haven't decided if I want to try to just outscale the pain in performance that comes w/ the MMP
branch, or if I want to try to just knock out `2^64%`[1]. I'm leaning MMP but I have to make sure
I've got that dog in me.

So, naturally I turn to the other thing, parallelization.

I have mostly implemented the thing, but I'm chasing the borrow-checker around and I need to work
out how this thing is going to function.

I want to make sure that if the program crashes, I can recover with minimal lost work. We can see
why in a bit from `jhg`'s work, they note:

> We tested the first 1x10^15 (1 quadrillion) values over a period of ~5 months (January-May, 2020),
> but no additional solutions were found.

5 months is a long time, and indeed the time will grow exponentially for me. I'm attacking that
problem through scaling out, but I also need to consider how often I want to spend 'saving' the
work I've already done so that I don't waste it in an unexpected crash.

I also want to be able to upgrade this over time, I'm going to be continuing to run this well past
the initial `2^64%` goal, so I want to be able to reliably stop without wasting work. My plan is to
build to the point where I have _some_ implementation going that can reliably generate new NSWs as
quickly as possible, so that I can then slowly iterate and replace parts as it runs.

To accomplish this, I really want to think in terms of an RPO, not in terms of how it's going to
work; if I never want to lose more than some target time -- say a minute -- of data, then it's easy
enough to break off a 'chunk' of my target range and search it; identifying some set of primes just
outside the range dynamically, it can employ any number of algorithms for searching that range that
it likes, so long as I record:

1. The primes used
2. The number of solutions/nonsolutions in the range
3. The SHA of the version of code being run, so we can replicate it after the fact
4. Build information (dependencies, etc) associated with the code

A future skeptic could then check out the same code at the same versions, compile the code, and with
reasonable certainty replicate the chunk. 

I'd conjecture that larger chunks will generally be more efficient, but smaller chunks are safer
from a crash-perspective. Because I also expect to scale to new hardware in this process, I won't
always know how large of a range will take a minute to process, so ultimately I want to have some
dynamic 'tuning' capability.

Since I'm already breaking things into chunks, it makes sense to me to have a Broker/Worker design
to fan this work out across cores. The 'broker' hands out 'chunks' to the 'spans' along with some
primes that are just slightly outside the span. I'm not sure that'll make things faster, but it's
very aesthetic.

I've roughed in a version of that, but it's still missing a couple pieces.

First, I'm pretty sure as written it'll only run one thread; but I was fighting a lot with the
borrow checker and wanted to get to some psuedo-stable point.

Second, I haven't implemented the 'take_primes' function which should return a fixed number of
primes upon request.

I have a lot of ideas for optimizations, but I want to get to the point where I can start running
this in some capacity and start figuring out how to upgrade it in situ.


----


[1] An footnote: I am going to name some things. 

First, an acronym: WNS or NSW, both mean `Witness of Nonsolution` or `Nonsolution-Witness`. These
are the primes for which the Legendre test returns a non-residue result.

Second, I'm going to assign some 'speedrun' categories to different bounds. The obvious ones are:

1. Gupta% : Wall time to 63
2. Berndt-Galway% : Wall time to 1e9
3. Matson% : Wall time to 1e12
4. jhg% : Wall time to 1e15
5. 2^64% : Wall time to solve for every value less than 2^64 
6. Brocard% : Time To Novel Proof there are/are no more solutions.

Other categories may be introduced in the future. I have assigned names as seem appropriate, the
first person to set a record in the category should, I think, have a right to name it, so if any of
these fine folks want to change the name I'm happy to oblige as official self-nominated benevolent
record-keeper for as long as the position suits me.

Thank you for coming to my footnote.

