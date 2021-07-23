/** What I learned:
    The triangular number is the additive analog of factorial:
        https://mathworld.wolfram.com/TriangularNumber.html
    An O(n) way to check if two numbers(i) can sum to equal a third(n) is to
        add n-i to a hashset and see if i is already in the list:
        https://stackoverflow.com/a/21792886/5098999
**/
using System;
using System.Linq;

public ulong GetTriangularNumber(ulong n)
{
    ulong sum = 0;
    for (ulong i = n; i > 0; i--)
    {
        sum += i;
    }
    return sum;
}

public ulong Get23Answer()
{
    ulong[] abundantNumbers = GetAbundantNumbers(28123, 1);
    ulong sum = GetTriangularNumber(23);

    for (ulong i = 25; i <= 28123; i++)
    {
        HashSet<ulong> sumCompleters = new HashSet<ulong>();
        bool summable = false;

        foreach (ulong abundantNumber in abundantNumbers)
        {
            var difference = i - abundantNumber;
            if (difference >= 0)
            {
                sumCompleters.Add(i - abundantNumber);
                if (sumCompleters.Contains(abundantNumber))
                {
                    summable = true;
                    break;
                }
            }
        }

        if (!summable)
        {
            sum += i;
        }
    }

    return sum;
}

public ulong[] GetAbundantNumbers(ulong limit, ulong start = 1)
{
    IList<ulong> abundants = new List<ulong>();

    for (ulong i = start; i <= limit; i++)
    {
        if (GetAbundancy(i) == Abundance.Abundant)
        {
            abundants.Add(i);
        }
    }

    return abundants.ToArray();
}

public enum Abundance
{
    Deficient,
    Perfect,
    Abundant
}

public Abundance GetAbundancy(ulong n)
{
    switch (GetProperDivisors(n).Aggregate((a, c) => a + c))
    {
        case ulong x when x == n:
            return Abundance.Perfect;
        case ulong x when x > n:
            return Abundance.Abundant;
        case ulong x when x < n:
            return Abundance.Deficient;
        default:
            throw new Exception("impossible!");
    }
}

public ulong[] GetProperDivisors(ulong n)
{
    IList<ulong> divisors = new List<ulong>() { 1 };

    for (ulong i = 2; i <= n / 2; i++)
    {
        if (n % i == 0)
        {
            divisors.Add(i);
        }
    }

    return divisors.ToArray();
}

Console.WriteLine(Get23Answer());
