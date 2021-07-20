/** What I learned:
    (i forgot how at first but) When initializing an object you can set values in line:
        new List<int>(){2};
    C# 9 supports native int types:
        https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/nint-nuint
        This allows to just specify the largest available and not worry about
        bitness.
**/
using System;

// Get all prime numbers from 2 - n/2
public ulong[] GetPrimes(ulong limit)
{
    // Add guard clause
    System.Collections.Generic.List<ulong> primes = new List<ulong>() { 2 };

    for (ulong i = 3; i <= limit; i++)
    {
        for (ulong j = 2; j < i; j++)
        {
            if (i % j == 0)
            {
                break;
            }
            if (j == i - 1)
            {
                primes.Add(i);
            }
        }
    }

    return primes.ToArray();
}

public enum Direction
{
    Up,
    Down
}
public ulong GetNextPrime(ulong number, Direction direction)
{
    for (ulong i = number; i >= 2 && i <= ulong.MaxValue; i = direction == Direction.Down ? i - 1 : i + 1)
    {
        for (ulong j = 2; j < i; j++)
        {
            if (i % j == 0)
            {
                break;
            }
            if (j == i / 2)
            {
                return i;
            }
        }
    }
    throw new Exception("Prime not found!");
}

public bool IsPrime(ulong number)
{
    for (ulong j = 2; j < number; j++)
    {
        if (number % j == 0)
        {
            return false;
        }
        if (j == number - 1)
        {
            return true;
        }
    }
    return false;
}

public ulong GetLargestPrimeFactor(ulong number)
{
    ulong maxNumerator = number;
    List<ulong> factors = new List<ulong>();

    for (ulong i = 2; i < maxNumerator / 2; i++)
    {
        if (maxNumerator % i == 0)
        {
            maxNumerator = maxNumerator / i;
            factors.Add(i);
            continue;
        }
        else if(i == maxNumerator / 2 - 1)
        {
            factors.Add(maxNumerator);
        }
    }

    foreach (ulong factor in factors.OrderByDescending(x => x))
    {
        if (IsPrime(factor))
        {
            return factor;
        }
    }

    throw new Exception("No prime factors!");
}

Console.WriteLine(GetLargestPrimeFactor(600851475143));
