use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Game {
        token TOP { 'Game '<number>': '<game> }
        token game { <set>['; '<set>]* }
        token set { <cube-set>[', '<cube-set>]* }
        token cube-set { <number> ' ' <color> }
        token number { \d+ }
        token color { 'red' || 'green' || 'blue' }
    }

    my %max_cubes =
        'red' => 12,
        'green' => 13,
        'blue' => 14,
    ;

    my $part-one-sum = 0;
    my $part-two-sum = 0;
    for $file.lines -> $line {
        Game.parse($line);
        my $valid-game = True;
        for $/<game><set> -> $set {
            for $set<cube-set> -> $cube-set {
                if +$cube-set<number> > %max_cubes{$cube-set<color>} {
                    $valid-game = False;
                }
            }
        }
        if $valid-game {
            $part-one-sum += +$/<number>;
        }

        my %minimum-cubes;
        for $/<game><set> -> $set {
            for $set<cube-set> -> $cube-set {
                %minimum-cubes{$cube-set<color>} max= +$cube-set<number>;
            }
        }
        $part-two-sum += [*] %minimum-cubes.values;
    }
    say "part 1: $part-one-sum";
    say "part 2: $part-two-sum";
}