# 789
# 456
# 123
#  0A
my @keypad is List = (<7 8 9>, <4 5 6>, <1 2 3>, <~ 0 A>); say @keypad.raku;

#  ^A
# <v>
my @dpad is List = (<~ ^ A>, <\< v \>>); say @dpad.raku;

my %keypad-lookup is Map = create-lookup(@keypad);
my %dpad-lookup is Map = create-lookup(@dpad);

sub MAIN($input) {
    my $file = open $input;
    my @codes = $file.slurp.trim.lines>>.comb>>.List;

    my $part1-solution = 0;
    my $part2-solution = 0;
    for @codes -> @code {
        my $robot-a-start = "A";
        my $code-final-length = 0;
        my $min-santa-choices-part1 = find-min-code-choices(%keypad-lookup, @code, 3);
        my $min-santa-choices-part2 = find-min-code-choices(%keypad-lookup, @code, 26);
        $part1-solution += $min-santa-choices-part1 * @code[0..*-2].join.Int;
        $part2-solution += $min-santa-choices-part2 * @code[0..*-2].join.Int;
    }
    say "part1 solution: $part1-solution";
    say "part2 solution: $part2-solution";
}

my %min-choice-tracker;

sub find-min-code-choices(%lookup, @code, $levels) {
    return @code.elems if $levels == 0;
    return %min-choice-tracker{"$levels;{@code.join}"} if %min-choice-tracker{"$levels;{@code.join}"}:exists;
    my $start = "A";
    my $code-final-length = 0;
    for @code -> $c1 {
        my @choices = find-path-choices(%lookup, $start, $c1);
        $start = $c1;

        my $min-choice = Nil;
        for @choices -> @choice {
            $min-choice min= find-min-code-choices(%dpad-lookup, @choice, $levels -1);
        }
        
        $code-final-length += $min-choice;
    }
    %min-choice-tracker{"$levels;{@code.join}"} = $code-final-length;
    return $code-final-length;
}

sub create-lookup(@pad) {
    my %lookup;
    for ^@pad.elems X ^@pad[0].elems {
        %lookup{@pad[.[0]][.[1]]} = $_;
    };
    return %lookup;
}

sub find-path-choices(%lookup, $start, $end) {
    my @start := %lookup{$start};
    my @end := %lookup{$end};
    my @invalid := %lookup<~>;
    my @vertical;
    if @start[0] < @end[0] {
        @vertical := ("v" xx (@end[0] - @start[0])).List;
    } elsif @start[0] > @end[0] {
        @vertical := ("^" xx (@start[0] - @end[0])).List;
    }
    my @horizontal;
    if @start[1] < @end[1] {
        @horizontal := (">" xx (@end[1] - @start[1])).List;
    } elsif @start[1] > @end[1] {
        @horizontal := ("<" xx (@start[1] - @end[1])).List;
    }
    if @horizontal.elems == 0 || @vertical.elems == 0 {
        return ((slip(@vertical), slip(@horizontal), "A"),);
    } elsif @start[0] == @invalid[0] && @end[1] == @invalid[1] {
        return ((|@vertical, |@horizontal, "A"),);
    } elsif @start[1] == @invalid[1] && @end[0] == @invalid[0] {
        return ((|@horizontal, |@vertical, "A"),);
    } else {
        return ((|@vertical, |@horizontal, "A"), (|@horizontal, |@vertical, "A"));
    }
}