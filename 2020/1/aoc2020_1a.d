import std.stdio;
import std.file;
import std.algorithm;

void main()
{
    // Let's get going!
    writeln("Hello World!");

    int[] a = slurp!(int)("input.txt","%s");
    a.sort();
    auto low_i = 0;
    auto high_i = a.length - 1;
    
    while (low_i != high_i) {
        int low = a[low_i];
        int high = a[high_i];
        int sum = low + high;
        if (sum == 2020) {
            break;
        } else if (sum < 2020) {
            low_i += 1;
        } else {
            high_i -= 1;
        }
    }
    writeln(a[low_i]);
    writeln(a[high_i]);
    writefln("prod: %d", a[low_i] * a[high_i]);
}
