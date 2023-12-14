# Project Euler

![Project Euler User Profile Badge For Nick_Kimbrough](https://projecteuler.net/profile/Nick_Kimbrough.png)

My Solutions to [ProjectEuler.net](https://projecteuler.net/)'s Problems. This repository will contain
spoilers to the first 100 questions, and is intended to show the methods I used
to come to the answers, in line with [Project Euler's Rules](https://projecteuler.net/about#publish) (must be logged in to read).

I do this primarily for fun, and to help me keep my skills sharp and learn new
languages and algorithms or mathematical methods I may not have known about.

## Running The Solutions

I have used VSCode as my editor of choice for all of the solutions posted here.
Certain languages require additional setup in order to run.

### C\#

1. Make sure you have the
[.NET Core SDK](https://dotnet.microsoft.com/download) (2.1 minimum) installed.
2. Install the dotnet-script tool with the following command:
    - `dotnet tool install -g dotnet-script`
3. Ensure the path to the dll is correct in `.vscode\launch.json`.

### Rust

Make sure you have the recommended extensions from `./vscode/extensions.json`
installed, along with [Rust](https://www.rust-lang.org/tools/install). To run
or debug, click the buttons above the main function.
