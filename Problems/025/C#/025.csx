/**
Initial Thoughts:
Use BigInteger to store a large number containing 1000 digits.

I think I could write a Fibonacci sequence generator that will yield each
result in the sequence until stopped. This might come in handy later.

More on yielding: https://medium.com/pvs-studio/what-is-yield-and-how-does-it-work-in-c-d69d1deab60
(wow they're doing a Fibonacci generator as well!)
Brian Lagunas has a good video on yielding: https://www.youtube.com/watch?v=HRXkeaeImGs

Here is my initial idea:

IEnumerator<BigInteger> GenerateFibonacciSequence() 
{
    BigInteger currentFibonacciNumber = 1;
    // Do complicated Fibonacci generation here
    yield return 1;
}

Run the generator and iterate over the result until the length of the output
converted to a string is n (in the problem's case, 1000).

These thoughts and some tinkering eventually ended up with this function:
**/

using System.Numerics;

// Let n equal the number of digits specified in the problem
int n = 1000;

IEnumerable<BigInteger> GenerateFibonacciSequence()
{
    BigInteger previousFibonacciNumber = 0;
    BigInteger currentFibonacciNumber = 1;

    while (true)
    {
        yield return currentFibonacciNumber;

        BigInteger newFibonacciNumber = previousFibonacciNumber + currentFibonacciNumber;
        previousFibonacciNumber = currentFibonacciNumber;
        currentFibonacciNumber = newFibonacciNumber;
    }
}

/**
We should now be able to call this function to generate the sequence, then
iterate over it to get the answer to the problem.
**/

IEnumerable<BigInteger> fibonacciSequence = GenerateFibonacciSequence();

BigInteger index = 1;
foreach (BigInteger fibonacciNumber in fibonacciSequence)
{
    if (fibonacciNumber.ToString().Length >= n)
    {
        Console.WriteLine($"The answer is {fibonacciNumber}, located at index {index}");
        break;
    }
    index++;
}

/** What I learned:
  Yield in C# can result in massive performance and memory savings, allowing for
  us to iterate over difficult objects much faster than traditional methods.
**/

/** Thoughts on improvement:
  This method uses a generator to generate and check every number in the
  Fibonacci sequence until we find the right one. We could use some more
  advanced techniques to get the Fibonacci number at a certain index using
  Phi  and then create windows to search to potentially speed this up. Just for
  generating the first one that is 1000 characters long though this suffices.
**/