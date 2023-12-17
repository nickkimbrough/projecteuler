/**
Initial Thoughts:

Right away I know that we will be dealing with 10 digits of 10 possible values,
so the number of permutations we will be dealing with is 10!, or 3,628,800.

Since there can only be one of each number, maybe some sort of string
manipulation would make sense here.

Thinking about how to generate the smallest number, we would grab the smallest
numbers in order, and reverse for the largest. To get the next number in the
order after the smallest....

Smallest -          0123456789
Next smallest -     0123456798  swap 10 with 9
Next -              0123456879  swap 10 with 9 then 9 with 8

This is feeling a lot like binary counting, but every bit can hold 10 different
values rather than 2. So this would be decimal counting....which I guess makes
sense! 😆

This binary counting is how it feels like the numbers are switching almost
though:

0000000001
0000000010
0000000011
0000000100
0000000101
0000000111
0000001000

I feel like the leading 1 in this position is related to where the 9 is at in
the position above.... but after thinking more about it, this is basically a
permutation problem like outlined here:

https://stackoverflow.com/questions/4240080/generating-all-permutations-of-a-given-string

This led me to Wikipedia: https://en.wikipedia.org/wiki/Permutation#Algorithms_to_generate_permutations
most notably the most interesting Generation in lexicographic order. This sounds
like EXACTLY what we need.

Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation is the last permutation.
Find the largest index l greater than k such that a[k] < a[l].
Swap the value of a[k] with that of a[l].
Reverse the sequence from a[k + 1] up to and including the final element a[n].

Some googling about it resulted with this https://stackoverflow.com/a/35727409,
but this assumes you already have the permutations and is just some basic LINQ
ordering. I need to GENERATE the permutations.

A bit more googling around and I found this (look familiar?): 
https://stackoverflow.com/a/11208543/5098999

It looks like someone took his work even further and multi threaded it:
https://stackoverflow.com/a/50569606/5098999

So here is my idea: learn from those who have come before me, but write this
solution myself so I fully understand it.
**/

IEnumerable<string> GeneratePermutations(int[] inputList)
{
    yield return string.Join("", inputList.Select(p => p.ToString()));

    int[] numList = inputList;
    while (true)
    {
        int largestIndex = -1;
        for (int i = numList.Length - 2; i >= 0; i--)
        {
            if (numList[i] < numList[i + 1])
            {
                largestIndex = i;
                break;
            }
        }

        if (largestIndex < 0)
        {
            break;
        }

        var largestIndex2 = -1;
        for (var i = numList.Length - 1; i >= 0; i--)
        {
            if (numList[largestIndex] < numList[i])
            {
                largestIndex2 = i;
                break;
            }
        }

        var tmp = numList[largestIndex];
        numList[largestIndex] = numList[largestIndex2];
        numList[largestIndex2] = tmp;

        for (int i = largestIndex + 1, j = numList.Length - 1; i < j; i++, j--)
        {
            tmp = numList[i];
            numList[i] = numList[j];
            numList[j] = tmp;
        }

        yield return string.Join("", numList.Select(p => p.ToString()));
    }
}

int[] input = new[] {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

IEnumerable<string> results = GeneratePermutations(input);

// Get the answer classically (very slightly faster than LINQ):
int index = 1;
foreach (string result in results)
{
    if (index == 1000000)
    {
        Console.WriteLine($"The answer is {result}");
    }
    index++;
}

// LINQ alternative:
Console.WriteLine(results.Take(1000000).Last());

/** What I learned:
  Generating permutations in lexicographic order dates back all the way to the
  14th century in India! Narayana Pandita was the first known to discover it.

  This seems to be a common interview question.

  I really like using yielding generator functions in C#!
**/

/** Thoughts on improvement:
  We could use the multithreaded improvements mentioned in the StackOverflow
  post. We could skip ahead as we know a lot of the lower numbers wouldn't be
  correct.
**/