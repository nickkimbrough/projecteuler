/** What I learned:
  I used to be fascinated with a similar spiral, Ulam's spiral:
  https://en.wikipedia.org/wiki/Ulam_spiral
**/

public int[] GetDiagonals(int n)
{
    if (n % 2 == 0)
    {
        throw new Exception("n must be odd!");
    }
    System.Collections.Generic.List<int> diagonals = new();
    diagonals.Add(1);

    int sum = 1;
    int additive = 2;
    for (int i = 0; i < Math.Round(((n - 1d) / 2d), MidpointRounding.AwayFromZero); i++)
    {
        for (int j = 0; j < 4; j++)
        {
            sum += additive;
            diagonals.Add(sum);
        }

        additive += 2;
    }

    return diagonals.ToArray();
}

// get numbers between 1 and n, where n is the width of the square spiral
int n = 1001;
int[] diagonals = GetDiagonals(n);
Console.WriteLine(diagonals.Sum());
