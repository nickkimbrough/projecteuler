/** What I learned:
In C#, a for loop doesn't HAVE to have a condition section:
    https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/statements/iteration-statements#the-for-statement
In math, Phi is used to specify the Golden ratio. It is (1+√5)/2.
    The C# Math library doesn't contain a constant for this:
        https://stackoverflow.com/a/24764630/5098999
In the Fibonacci sequence, every third value is even due to the way the numbers
are added.
**/
using System;

readonly double Phi = (1 + Math.Sqrt(5)) / 2;
int sum = 0;
for (int i = 3;; i+= 3)
{
    int value = (int)((Math.Pow(Phi, i) - (-Math.Pow(Phi, -i))) / Math.Sqrt(5));

    if (value > 4000000)
    {
        break;
    }

    sum += value;
}

Console.WriteLine(sum);
