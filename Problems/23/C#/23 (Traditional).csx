/** What I learned:

**/
using System;
using System.Linq;

public ulong[] GetAbundantNumbers(ulong limit, ulong start = 1)
{
    throw new NotImplementedException();

    System.Collections.Generic.List<ulong> abundants = new List<ulong>();

    for (ulong i = start; i < limit; i++)
    {
    }

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
    IList<ulong> divisors = new List<ulong>();

    for (ulong i = 1; i <= n / 2; i++)
    {
        if (n % i == 0)
        {
            divisors.Add(i);
        }
    }

    return divisors.ToArray();
}

Console.WriteLine(GetAbundancy(119));
