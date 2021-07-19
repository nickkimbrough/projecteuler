/** What I learned:
    (i forgot how but) When initializing an object you can set values in line:
        new List<int>(){2};
**/
using System;

// Get all prime numbers from 2 - n/2
public int[] GetPrimes(int limit) {
    // Add guard clause
    System.Collections.Generic.List<int> primes = new List<int>(){2};

    for (int i = 3; i <= limit; i++)
    {
        for (int j = 2; j < i; j++)
        {
            if (i%j == 0)
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
