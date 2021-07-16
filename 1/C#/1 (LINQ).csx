using System;

Console.WriteLine(Enumerable.Range(1, 999).Where(number => number % 3 == 0 || number % 5 == 0).Sum());
