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
least one value that passed 40 tests. This repo instead has a pool of 1000 primes, and simply tests
at random until it gets a definite-negative result.

### Why Sampling instead of a list, like prior work?

Sampling at random means we'll need to remember which primes we've tested against for each batch,
but it also means that we can scale to arbitary size, and continually run the test until we get a
definite negative, limited only by the size of the relevant primes.

We need the primes to be large enough to ensure they do not divide `n! + 1`, which means we must
have `p > n`, or else the symbol is 0, which is not useful. The pool of values contains all prior
values from both
[Matson](https://web.archive.org/web/20181006100943/http://unsolvedproblems.org/S99.pdf) and
[jhg023](github.com/jhg023/brocard)'s work, as well as several thousand other values of varying
sizes. The algorithm here will filter the prime list to only those primes which may be used, and 
if it cannot complete it's batch with the primes it has available, it will report a possible
positive in it's output. This can then be retested with a different pool of primes or otherwise
handled.

## Using the Legendre Symbol to answer Brocard

The algorithm we use is otherwise exactly the same as
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



