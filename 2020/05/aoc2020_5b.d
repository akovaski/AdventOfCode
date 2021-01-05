import std.stdio;
import std.string;
import std.file;
import std.algorithm;
import std.array;
import std.conv;

void main()
{
    auto input_file = readText("input.txt").splitLines();

    bool[int] seat_ids;

    int highest_seat = -1;
    foreach(i, line; input_file) {
        string fb = line[0 .. 7];
        string lr = line[7 .. 10];

        int row = fb.map!(c => c == 'F' ? '0' : '1').to!int(2);
        int col = lr.map!(c => c == 'L' ? '0' : '1').to!int(2);

        seat_ids[row * 8 + col] = true;
    }

    foreach(seat_id; 8 .. 1016) {
        if (!(seat_id in seat_ids)) {
            writefln("Missing seat: %d", seat_id);
            break;
        }
    }
}
