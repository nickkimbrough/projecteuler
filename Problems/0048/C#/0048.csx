/**
Initial Thoughts:
This is going to grow exponentially and be hard to calculate. I think I will Use
BigInteger to store a large number containing 1000 digits. I did similar in
question 25. I will just use a for loop with a maximum value and add to the sum.
then, I will convert it to a string and get a substring to print out to the
console.

**/

using System.Numerics;

// Let n equal the number of digits specified in the problem
int n = 1000;

string GetSelfPowers(int iterations)
{
    BigInteger sum = 0;

    for (BigInteger i = 1; i < iterations; i++)
    {
        sum += BigInteger.Pow(i, (int)i);
    }

    var sumString = sum.ToString();
    return sumString.Substring(sumString.Length - 10);
}

Console.WriteLine(GetSelfPowers(n));


/** What I learned:
  BigInteger allows for massive computations like this.
**/

/** Thoughts on improvement:
  We are limited by the size of n to int.MaxValue
**/