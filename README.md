> [!CAUTION]
> This is _very_ work in progress, especially the documentation part, so if you're reading
> this, please take it with a heavy grain of salt until this tip is removed.


# What the heck is this?

Brocard's Problem asks:

    n! + 1 = k^2 for (n,k) in S = {(4,5), (5,11), (7,71)}

    What is the cardinality of S?

[Prior work](https://github.com/jhg023/brocard) has been done to verify this up to 1e15. This was
done on a single machine over several months.

I'm an operations guy and a math guy, we can do better than a single threadripper and months of
computation, and if that's the case, then we can probably do better than 1e15 in 5 months.

This isn't a callout or anything, and I'm not honestly all that interested in the problem itself as
I am with setting up a system for efficient boundary busting on a home-built HPC cluster. I'm aware
of prior HPC tools, I'll probably use them someday, but for now the goal is to build up some code
and infrastructure to distribute a small program that can be run to push the boundary up another
order of magnitude (to 1e18).

Broadly speaking the approach is the same as the above repo, reimplemented in Rust, and distributed
across (at the moment) a pair of R730s. The goal is to have a system that will scale efficiently to
an arbitrary number of systems.

The first target for this project is to replicate
[Matson](https://web.archive.org/web/20181006100943/http://unsolvedproblems.org/S99.pdf)'s work,
which verified no new solutions less than 1e12; after that, we'll proceed to verify
[jhg023](https://github.com/jhg023/brocard)'s work, which verified no new solutions less than 1e15.
Ideally in quite a bit less than 5 months. Moore's Law would argue that since around 36 months have
past, we should be able to do it in at least 1/4th the time, so the target is to verify 1e12-1e15 in
5 _weeks_ instead of 5 months.

After that, we'll start pushing the boundary, with the confidence that we're at least as accurate as
prior work, and with some good data on how the system performs. The goal is to push the boundary up
at least one order of magnitude, but in all honesty, I suspect we'll try for more than that.

## What else is this?

This is also a prototype for solving exposition problems. The point of this code is not merely to
push a boundary, but to _explain how to push boundaries_. Ideally this code will be:

1. Thoroughly documented, more than typical code.
2. Thoroughly tested and benchmarked.
3. Thoroughly explained, both in implementation and concept.

Again, I'm not super interested in Brocard's problem except as a lever to aid in other things.
Building a home-grown scalable HPC system is a fun and interesting problem in it's own right,
learning how to leverage number theory to attack this kind of problem is valuable even if we don't
resolve the problem itself.

Lots of folks focus on pushing the edges of mathematical understanding, trying to reach new places
and expand their perception of reality. I'm not interested in that, I'm interested in staring very
carefully at the well-trod ground and trying to see if there is an improvement I can make to the
road others have walked to make it so the next folks can get there faster and with less effort.

In short, I'm here to do some scut-work and make the path a little easier for whoever comes after
me. It's not pretty, it's not powerful, it's not revolutionary, but it's honest work and I think we
ignore it at our peril.

# What's the plan?

1. Build this tool to do the computation across some range.
2. Build a tool to distribute running this across an arbitrary number of machines
3. Run the second tool to run the first tool to replicate the result from Prior Work above.
4. Run the second tool to run the first tool to push the boundary upwards

Ideally we calculate:

1. How many batches we can process per unit time.
2. The results calculated by the above (e.g., how many legendre tests it passes)

The underlying test we'll be using relies on a property of the legendre test. The test is inherently
probabilistic (see [HOW_IT_WORKS.md](HOW_IT_WORKS.md) for more details), so we need to run it
multiple times until we find a positive failure (indicating there is no solution for that `n`).
Positive failures become more rare as time `n` gets larger, so we'll need to run more tests
appropriately. In the prior work, at least one `n` required 40 tests before being marked as a
negative.

# What's in this repo?

A reimplementation of prior work, which will be optimized to process 'batches' of values based on
two input parameters, a 'start' and a 'span'. The 'start' is the first value to test, and the 'span'
is the number of values to test. The program will test all values from `start` to `start + span` and
record the results as a JSON object which is reported back to a separate service for logging. This
object will contain the candidate tested, how many tests it passed, and which primes it was tested
against.

The implementation will incorporate SIMD instructions and whatever other tools may make sense (I'd
love to see if there is room for, e.g., GPU accelleration, but I haven't done any research toward
that).

# What's the status?

Nascent, I've mostly been focusing on the run infrastructure (e.g., how to manage the configuration
of a fleet of machines consistently) using Nix and NixOS. That work is not yet public mostly because
I haven't had the time to extract all the sensitive bits from the repo and make it safe to share.

# LICENSE

This work is licensed under the GPLv3 license. See the LICENSE file for more details.


