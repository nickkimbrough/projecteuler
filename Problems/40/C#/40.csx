/** What I learned:
  Sometimes loops and strings beat math if you have the resources for it
**/
using System;
using System.Linq;

int n = 1000000;
StringBuilder sb = new StringBuilder();

for (int i = 1; sb.Length <= n; i++)
{
    sb.Append(i.ToString());
}

string finalNumber = sb.ToString();

int answer = 1;

for (int i = 10; i <= n; i = i * 10)
{
    answer = answer * Int32.Parse(finalNumber[i-1].ToString());
}

Console.WriteLine(answer);
