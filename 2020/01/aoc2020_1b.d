import std.stdio;
import std.file;
import std.algorithm;

void main()
{
    // Let's get going!
    writeln("Hello World!");

    int[] a = slurp!(int)("input.txt","%s");
    a.sort();
    ulong low_i = 0;
    ulong high_i = 0;
    ulong third_i = 0;
    foreach(third; 0..a.length) {
        low_i = 0;
        high_i = a.length - 1;

        while (low_i != high_i) {
            int low = a[low_i];
            int high = a[high_i];
            int sum = low + high + a[third];
            if (sum == 2020) {
                break;
            } else if (sum < 2020) {
                low_i += 1;
            } else {
                high_i -= 1;
            }
        }
        if (a[low_i] + a[high_i] + a[third] == 2020) {
            third_i = third;
            break;
        }
    }
    writeln(a[low_i]);
    writeln(a[high_i]);
    writeln(a[third_i]);
    writefln("prod: %d", a[low_i] * a[high_i] * a[third_i]);
}
