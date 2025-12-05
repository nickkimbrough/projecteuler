/**Initial Thoughts:
 * 
 * I probably just need to write an efficient string
 * palindrome checker here, then pass string representations
 * of the base 10 and base 2 number.
 * 
 * I think I'll just split the string in two
 * (ignoring the middle character if odd length),
 * then use LINQ to reverse the string and compare them for
 * sameness.
 */

 /** Solution Found Thoughts:
 *  This went pretty much as I expected. When I went to write
 *  the palindrome checker, I realized I could just reverse
 *  the string and compare it to the original, which is
 *  simpler than my initial idea.
 */

 /** Forum thoughts:
  *  user zef
  *  pointed out that only odd numbers need to be checked,
  *  since even numbers in binary will end with a 0 and
  *  always begin with a 1, so they can't be palindromic.
  */

int GetDoubleBasePalindromes(int maxValue)
{
    int palindromeSum = 0;
    for (int i = 0; i <= maxValue; i++)
    {
        ICollection<string> numbers = [];
        numbers.Add(Convert.ToString(i, 2));
        numbers.Add(i.ToString());

        bool isPalindrome = true;
        foreach (string number in numbers)
        {
            if (number != new string([.. number.Reverse()]))
            {
                isPalindrome = false;
            } 
        }

        palindromeSum += isPalindrome ? i : 0;
    }
    return palindromeSum;
}

Console.WriteLine(GetDoubleBasePalindromes(999999));