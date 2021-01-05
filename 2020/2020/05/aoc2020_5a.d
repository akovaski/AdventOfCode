import std.stdio;
import std.string;
import std.file;
import std.algorithm;
import std.array;
import std.conv;

void main()
{
    auto input_file = readText("input.txt").splitLines();

    int highest_seat = -1;
    foreach(line; input_file) {
        string fb = line[0 .. 7];
        string lr = line[7 .. 10];

        int row = fb.map!(c => c == 'F' ? '0' : '1').to!int(2);
        int col = lr.map!(c => c == 'L' ? '0' : '1').to!int(2);

        int seat_id = row * 8 + col;

        highest_seat = max(highest_seat, seat_id);
    }
    writefln("Highest seat id: %d", highest_seat);
}
