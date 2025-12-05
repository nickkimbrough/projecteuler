/** Initial Thoughts:
*
*   There has to be something that limits the upper range of numbers that can
*   equal the sum of the factorial of their digits. Thinking on this, if I take
*   999 as an example...9!+9!+9! = 1,088,640, four whole more digits than the
*   original number. It seems any digit above 3 is going to scale faster so
*   there's definitely a limiting principle at play here. I think I'll attempt
*   to write an algorithm that just calculates them and return if it hasnt
*   found one for an exceedingly long time. I might be able to find out the
*   underlying mathematical principle if I do that.
*
*   I'll map factorials of digits 0-9 so I don't have to recalculate them in
*   loops. Then I'll just loop and do some simple math to check.
*/

/** Solution Found Thoughts:
*   I'm going to try and figure out the upper bound. Most of the forum
*   discussion is about exactly this.
*
*   bitRAKE correctly points out that:
*   9999999 is an easy upper limit to come up with.
*   7 times 9! is less than 9999999.
*
*   spaecious has the gold:
*   the upper bound is 7 x 9!, because all 8-digit
*   numbers (10,000,000 to 99,999,999) are greater than the maximum possible
*   result of any 8-digit number (f 99,999,999 = 2,903,040). After 7
*   digits, there can be no more that are equal because the distance between the
*   minimum candidate and the maximum possible result (which is smaller) only
*   increases.
*
*   this means the logical upper limit is 2,540,160.
*   With this in place, my code executes in 00:00:00.4925232.
*/

int GetDigitFactorialsSumInitialSolution()
{
    Dictionary<byte, int> factorialMap = [];
    for (byte i = 0; i < 10; i++)
    {
        factorialMap.Add(i, CalculateFactorial(i));
    }

    int digitFactorialsSum = 0;
    for (int i = 3; i <= 2540160; i++)
    {
        var digits = i
            .ToString()
            .ToCharArray()
            .Select(x => (byte)(x - '0'));

        int sum = digits
            .Sum(d => factorialMap
                .TryGetValue(d, out var value) ? value : 0
            );

        digitFactorialsSum += sum == i ? i : 0;
    }

    return digitFactorialsSum;
}

Console.WriteLine(GetDigitFactorialsSumInitialSolution());

// Derived from TheAlgorithms project C# factorial implementation found at
// https://github.com/TheAlgorithms/C-Sharp/blob/master/Algorithms/Numeric/Factorial.cs
// Made to output an int instead and be faster with specific input
static int CalculateFactorial(int inputNum)
{
    int result = 1;
    for (int i = 1; i <= (int)inputNum; i++)
    {
        result *= i;
    }

    return result;
}