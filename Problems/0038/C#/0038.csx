/**Initial Thoughts:
 * 
 * Pandigital numbers have to range from 123456789
 * to 987654321. This leaves us with 864,197,532
 * possible pandigital numbers.
 * 
 * My idea is to do a greedy algorithm since we
 * only want the largest number. I'll start at
 * 987654321 and loop downwards, checking for
 * pandigitalness along the way. First hit I get,
 * that's my answer!
 * 
 * Thinking about it some more, we have algebraic
 * formulas where we can solve for X in our table.
 * I can break the pandigital number down and see
 * if it equates in all situations.
 */

/** Working Thoughts:
 * 
 *  The sizes won't be the same, can't break it that
 *  way, but I can check for it by grabbing the
 *  first digit, then two, then three, etc until
 *  I find a solution to the equation or move on.
 */

/** Final Thoughts:
  * 
  *  This worked pretty well. I had to do some
  *  string manipulation to get the pandigital
  *  checking working correctly, but otherwise
  *  it was straightforward.
  */
int GetLargestPandigital()
{
    for (int i = 987654321; i >= 0; i--)
    {
        string pandigitalString = i.ToString();
        // Check for pandigitalness
        if (pandigitalString.Replace("0", "").Length == 9 && pandigitalString.ToCharArray().Distinct().Count() == 9)
        {
            // Try and solve the equation with the number growing from left to right
            for (int j = 1; j <= 8; j++)
            {
                int pandigitalBase = int.Parse(pandigitalString[..j]); // 192
                string pandigitalWindow = pandigitalString[..j];
                for (int n = 2; ; n++)
                {
                    int nProduct = pandigitalBase * n;
                    if (!pandigitalString[pandigitalWindow.Length..].StartsWith((nProduct).ToString()))
                    {
                        break;
                    }
                    pandigitalWindow += nProduct.ToString();
                }

                if (pandigitalWindow == pandigitalString)
                {
                    return i;
                }
            }
        }
    }
    
    return 0;
}

Console.WriteLine(GetLargestPandigital());