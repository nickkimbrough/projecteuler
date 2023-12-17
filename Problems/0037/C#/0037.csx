/** Initial thoughts:
  Starting at  8, check to see if a number is prime. If it is, check to see if
  it is a truncatable prime. Once we have found 11 truncatable primes, we can
  stop.
**/
using System;
using System.Linq;
using Internal;

int sum = 0;
int count = 0;

for (int i = 8; count < 11; i++)
{
  bool isTruncatable = IsPrime(i) ? IsTruncatable(i) ? true : false : false;

  if (isTruncatable)
  {
    count += 1;
    sum += i;
  }
}

bool IsPrime(int number)
{
  if(number == 1) return false;
  for (int i = number-1; i > 1; i--)
  {
    if (number % i == 0)
    {
      return false;
    }
  }
  return true;
}

bool IsTruncatable(int number)
{
  string numberString = number.ToString();
  for (int i = 1; i < numberString.Length; i++)
  {
    string left = numberString.Substring(0, i);
    string right = numberString.Substring(i, numberString.Length - i);

    if (!IsPrime(Int32.Parse(left)) || !IsPrime(Int32.Parse(right)))
    {
      return false;
    }
  }
  return true;
}

Internal.Console.WriteLine(sum.ToString());

/** What I learned:
  These 'truncatable primes' are also known as 'two-sided primes':
  https://prime-numbers.info/article/two-sided-primes. There are only 15 of them
  known to exist.

  This problem was almost too difficult to brute force, with an older processor
  I feel like it would have taken a very long time to find the answer.

  Also, I remembered that 1 is not a prime number after running into some
  issues.

  Github Copilot helped a lot with this problem, but also spoiled some of the
  thought process, but thankfully to some of it's hints, I learned about the
  Sieve of Eratosthenes, which I will be using in the future.
**/
