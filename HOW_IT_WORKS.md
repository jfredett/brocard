# How to determine if a big number is a perfect square

We are dealing with extremely large numbers; `n!` grows faster than most other functions, and we are
asking a kind of question that rapidly exceeds the bounds of our machines, so we must rely on other
tools to help us.

Polya says, if you have a hard problem you can't solve. Find a simpler version of problem you also
can't solve, and try that first. [This
paper](https://web.archive.org/web/20181006100943/http://unsolvedproblems.org/S99.pdf) by Matson
does just that for Brocard.

To understand how it works, we'll need to understand the Legendre Symbol, and once we understand
that, we can see how we can leverage it to answer the question Brocard's Problem asks.

## The Legendre Symbol

The Legendre Symbol is a way to determine if a number is a perfect square. It is defined as:

- 1 if a is a perfect square mod p
- 0 if a is divisible by p
- -1 if a is not a perfect square mod p

> [!NOTE]
> For simplicity, the code actually represents the the "Nonresidue" result (`-1`) as `2`, so
> that all enum values are positive. I justify this by noting that `2 =~ 1 mod 3`.

The Legendre Symbol has a closed form calculation:

    L(a,p) = a^((p-1)/2) mod p

We'll see later how we can exploit some optimizations beyond simple modular
exponentiation/multiplication to make this computation very quick, but for now we're concerned
mostly with the mathematics, not the arithmetic.

This test is 'half-probabilistic', in that if a is a perfect square mod p, then the test will always
return 1, but it can _also_ return one if the value is _not_ a perfect square mod p. However, it
will _never_ return any other value if the value is a perfect square mod p.

What this means is that a number can 'pass' this test for many choices of `p`, and so long as it
fails for _at least_ one test, we can rule out that value as a perfect square, and thus as a
solution to Brocard. Let's do a quick example:

Suppose we want to know if `15` is a solution. In this case, we can directly calculate `15! + 1 =
1307674368001` and see readily that this is not a perfect square. However, we can also use the
Legendre test by fixing some prime `p`, let's say `1,000,000,000,039`, and calculating:

    L(15! + 1, 13) = (15! + 1)^((1000000000039-1)/2) mod 1000000000039
                   = 1307674368001^(500000000019) mod 100000000039
                   = 1

While we know (by inspection) that `15` is not a solution, we can also see that the Legendre test
does not produce a conclusive negative. Essentially the symbol says, "This might be a solution", and
in fact it will say this with probability very close to 50%. The above paper solves this by simply
checking against a table of 40 primes.

Later work in [this repo](github.com/jhg023/brocard) uses an additional 10 primes account for at
least one value that passed 40 tests. This code instead generates a bunch of primes for each run,
and so is configurable to generate more primes as needed. In the event we get back a report that a
number passes all the primes available, we can then review it for additional inspection.

## Using the Legendre Symbol to answer Brocard

The algorithm we use is exactly the same as
[Matson](https://web.archive.org/web/20181006100943/http://unsolvedproblems.org/S99.pdf)'s and
[jhg023](github.com/jhg023/brocard)'s work. We simply test a range of primes, looking for a definite
negative. As soon as we find one, we move on to the next. After testing every value in the range, we
build a result object and dump it to STDOUT.

In our example above using `n = 15`, we might have output like:

```jsonc
{
    15: {
        primes_tested: [1000000000039, 1000000000061, 1000000000063, ...],
        witness: 1000000000063,
    },
    # ...
}
```

These objects can then be imported, recorded, and reported on as needed.

The algorithm itself will run against many primes simultaneously via SIMD; at least up until 2^64,
at which point we'll need to switch to a multiprecison approach since SIMD doesn't support values
with more than 64 bits in rust at the moment.

> [!NOTE]
> I suppose I _could_ embrace the madness that is `core::arch` instead of using the nicer
> `core::simd`, but I do not wish to stare into that abyss until all other options are exhausted.



