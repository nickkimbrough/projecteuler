/** Initial Thoughts:
*
*   The first part is getting all of the primes below
*   a million. I'm pretty sure I've already done that in
*   a previous problem, so I'll use that or an algorithm
*   from TheAlgorithms if not.
*   
*   I'll then loop over each prime below a millon and
*   do string manipulation to create an collection
*   of numbers to check for primeness.
*   
*   I'll check each one of these and if they are all
*   prime, I'll add it to the count.
*/

/** Solution Found Thoughts:
 * 
 *  This worked basically like I imagined. Rotating
 *  was a bit more difficult than I thought of at
 *  first, because the leading 0 problem.
 */

/** Grading thoughts:
 *  I asked Gemini to grade me here and it pointed
 *  out a few areas of improvement:
 *  
 *  Prime checking could be optimized with a sieve
 *  of Eratosthenes.
 *
 *  I could have stored my primes in a HashSet for
 *  O(1) lookup time instead of O(n) with a list,
 *  and then I wouldn't have needed to check each
 *  rotation individually.
*/

int GetCircularPrimes(int maxValue)
{
    // First, get the prime numbers
    ICollection<int> primes = [];
    for (int i = 0; i <= maxValue; i++)
    {
        if (IsPrime(i))
        {
            primes.Add(i);
        }
    }

    // Now, do the comparison loop
    int circularPrimeCount = 0;
    foreach (int prime in primes)
    {
        // Get rotations
        string primeString = prime.ToString();
        ICollection<int> rotations = [];
        for (int i = 0; i < primeString.Length - 1; i++)
        {
            string rotation = new([.. primeString.Skip(1), primeString[0]]);
            primeString = rotation;
            rotations.Add(int.Parse(rotation[0] == '0' ? rotation[1..] : rotation));
        }

        // Are all rotations prime?
        circularPrimeCount += rotations.Select(x => IsPrime(x)).All(x => x) ? 1 : 0;
    }

    return circularPrimeCount;
}

Console.WriteLine(GetCircularPrimes(1000000));

// Taken from https://github.com/TheAlgorithms/C-Sharp/blob/master/Algorithms/Numeric/PrimeChecker.cs
// and cleaned up for brevity.
static bool IsPrime(int number)
{
    if (number <= 1)
    {
        return false;
    }

    if (number <= 3)
    {
        return true;
    }

    if (number % 2 == 0 || number % 3 == 0)
    {
        return false;
    }

    for (int i = 5; i <= number / i; i = i + 6)
    {
        if (number % i == 0)
        {
            return false;
        }
        if (number % (i + 2) == 0)
        {
            return false;
        }
    }

    return true;
}
