use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Records {
        token TOP { <times> "\n" <distances> "\n"* }
        token times { "Time:" \s* <num>+%\s+ }
        token distances { "Distance:" \s* <num>+%\s+ }
        token num { \d+ }
    }

    my $records = Records.parse($file.slurp);

    my $part-one-solution = 1;
    for $records<times><num>».Int Z $records<distances><num>».Int -> $record {
        $part-one-solution *= count-ways-to-beat(|$record);
    }
    say "part 1: $part-one-solution";

    my $kerned-time = $records<times><num>.join.Int;
    my $kerned-distance = $records<distances><num>.join.Int;
    my $part-two-solution = count-ways-to-beat($kerned-time, $kerned-distance);
    say "part 2: $part-two-solution";
}

sub count-ways-to-beat($time, $record-distance) {
    # time = button + go
    # distance = go * button
    # 0 = go^2 - time * go + distance
    # go = (time +/- sqrt(time**2 - 4*distance))/2

    # don't think too hard:
    # if odd t then t/2 = x.5,
    #   so sqrt(t**2-4*d)/2 = 2.3 => result = 4
    #   and sqrt(t**2-4*d)/2 = 2.5 => result = 6
    #   therefore result = 2 * (sqrt(t**2-4*d)/2 + 1/2).floor
    # even t then t/2 = x.0
    #   so sqrt(t^2-4*d)/2 = 2.x => result = 4 + 1(shared) = 5
    #   therefore result = 2 * (sqrt(t^2-4*d)/2).floor + 1
    # therefore result = 2 * ((sqrt(t**2-4*d)+t%2)/2).floor + 1 - t%2
    # Note: sqrt produces a Num, so perhaps the result could be off by 1 or 2,
    #       but it solved my AoC inputs correctly 😃.

    my $required-distance = $record-distance + 1;
    return 2 * ((sqrt($time**2 - 4*$required-distance) + $time%2)/2).floor + 1 - $time%2;
}