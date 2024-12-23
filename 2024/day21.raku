sub MAIN($input) {
    my $file = open $input;
    my @codes = $file.slurp.trim.lines>>.comb>>.List;

    # 789
    # 456
    # 123
    #  0A
    my @keypad is List = (<7 8 9>, <4 5 6>, <1 2 3>, <~ 0 A>); say @keypad.raku;

    #  ^A
    # <v>
    my @dpad is List = (<~ ^ A>, <\< v \>>); say @dpad.raku;

    say @codes.raku;


    my %keypad-lookup is Map = create-lookup(@keypad);
    my %dpad-lookup is Map = create-lookup(@dpad);
    my $part1-solution = 0;
    for @codes -> @code {
        # my @keypad-path = find-min-path(@keypad, @code, ("^", ">", "<", "v"));
        # my @all-code-choices;
        my $robot-a-start = "A";
        my $code-final-length = 0;
        for @code -> $c1 {
            my @robot-a-choices = find-path-choices(%keypad-lookup, $robot-a-start, $c1);
            $robot-a-start = $c1;

            
            # my @all-robot-a-choices;
            my $min-santa-choice-for-robot-a-choices = Nil;
            for ^@robot-a-choices.elems -> $i {
                my @robot-a-choice := @robot-a-choices[$i];
                # say "robot a choice: {@robot-a-choice.raku}";
                my $robot-b-start = "A";
                # my @all-robot-b-choices;
                my $robot-a-choice-length = 0;
                for @robot-a-choice -> $c2 {
                    # say "c2: $c2";
                    my @robot-b-choices = find-path-choices(%dpad-lookup, $robot-b-start, $c2);
                    $robot-b-start = $c2;

                    # say "robot b choices: {@robot-b-choices.raku}";
                    my $min-santa-choice-for-robot-b-choices = Nil;
                    for @robot-b-choices -> @robot-b-choice {
                        # say "robot b choice: {@robot-a-choice.raku}";
                        my $santa-start = "A";
                        my @santa-choices;
                        for @robot-b-choice -> $c3 {
                            @santa-choices.push(find-path-choices(%dpad-lookup, $santa-start, $c3));
                            $santa-start = $c3;
                        }
                        my $sc-length = @santa-choices.map({.[0].elems}).sum;
                        $min-santa-choice-for-robot-b-choices min= $sc-length;
                        # @all-santa-choices.push(($sc-length, @santa-choices.List));
                    }
                    $robot-a-choice-length += $min-santa-choice-for-robot-b-choices;
                    # my $min-all-santa-choices = @all-santa-choices.map({.[0]}).min;
                    # my @good-santa-choices is List = @all-santa-choices.grep({.[0] == $min-all-santa-choices});
                    # @all-robot-b-choices.push((@good-santa-choices.map({.[0]}).sum, @good-santa-choices));
                }
                $min-santa-choice-for-robot-a-choices min= $robot-a-choice-length;
                # my $min-all-robot-b-choices = @all-robot-b-choices.map(-> @santa-choices {@santa-choices[0]}).min;
                # my @good-robot-b-choices is List = @all-robot-b-choices.grep({.[0] == $min-all-robot-b-choices});
                # say "good robot b choices {@good-robot-b-choices.raku}";
                # @all-robot-a-choices.push((@good-robot-b-choices.map({.[0]}).sum, @good-robot-b-choices));
            }
            $code-final-length += $min-santa-choice-for-robot-a-choices;
            # say "all robot-a-choices {@all-robot-a-choices.raku}";
            # my $min-all-robot-a-choices = @all-robot-a-choices.map(-> @robot-b-choices {@robot-b-choices[0]}).min;
            # my @good-robot-a-choices is List = @all-robot-a-choices.grep({.[0] == $min-all-robot-a-choices});
            # say "good robot a choices <$min-all-robot-a-choices> {@good-robot-a-choices.raku}";
            # @all-code-choices.push((@good-robot-a-choices.map({.[0]}).sum, @good-robot-a-choices));
        }
        say "code final length: $code-final-length * {@code[0..*-2].join}";
        $part1-solution += $code-final-length * @code[0..*-2].join.Int;
        # say "all code choices {@all-code-choices.raku}";
        # my $min-all-code-choices = @all-code-choices.map(-> @robot-a-choices {@robot-a-choices[0]}).min;
        # say $min-all-code-choices;
        # say resolve-choices(@keypad-path);
        # say @keypad-path;
        # my @robot-a-path = find-min-path(@dpad, @keypad-path, ("v", ">", "<", "^"));
        # # say @robot-a-path;
        # my @robot-b-path = find-min-path(@dpad, @robot-a-path, ("v", ">", "<", "^"));
        # say @robot-b-path;
        # say @robot-b-path.elems;
    }
    say "part1 solution: $part1-solution";
    # say find-min-path(@dpad, "v<<AA>^AA>A".comb.List, ("v", ">", "<", "^"));
    # say find-min-path(@dpad, "v<<AA>^AA>A".comb.List, ("v", ">", "<", "^")).elems;
    #  ^A
    # <v>
    # 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A    <vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    #          <   A > A  v <<   AA >  ^ AA > A      v  AA ^ A   < v  AAA >  ^ A
    #              ^   A         <<      ^^   A         >>   A        vvv      A
    #                  3                      7              9                 A
    #
    # mine:             v<A<AA>>^AAvA<^A>AAvA^A
    #                     v <<   AA >  ^ AA > A
    #                            <<      ^^   A
    #                                         7
    # mine:             
    # 379A: v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A
    #          <   A > A   <   AA  v <   AA >>  ^ A  v  AA ^ A  v <   AAA >  ^ A
    #              ^   A       ^^        <<       A     >>   A        vvv      A
    #                  3                          7          9                 A
}

sub create-lookup(@pad) {
    my %lookup;
    for ^@pad.elems X ^@pad[0].elems {
        %lookup{@pad[.[0]][.[1]]} = $_;
    };
    return %lookup;
}

# sub find-min-path(@pad, @code, @order) {
#     my %lookup = create-lookup(@pad);
#     my @pos := %lookup<A>;
#     my @invalid := %lookup<~>;
#     my @path;
#     for @code -> $c {
#         my @new-pos := %lookup{$c};
#         my @vertical;
#         if @pos[0] < @new-pos[0] {
#             @vertical := ("v" xx (@new-pos[0] - @pos[0])).List;
#         } elsif @pos[0] > @new-pos[0] {
#             @vertical := ("^" xx (@pos[0] - @new-pos[0])).List;
#         }
#         my @horizontal;
#         if @pos[1] < @new-pos[1] {
#             @horizontal := (">" xx (@new-pos[1] - @pos[1])).List;
#         } elsif @pos[1] > @new-pos[1] {
#             @horizontal := ("<" xx (@pos[1] - @new-pos[1])).List;
#         }
#         my @choices;
#         if @horizontal.elems == 0 || @vertical.elems == 0 {
#             @choices := ((|@vertical, |@horizontal),);
#         } elsif @pos[0] == @invalid[0] && @new-pos[1] == @invalid[1] {
#             @choices := ((|@vertical, |@horizontal),);
#         } elsif @pos[1] == @invalid[1] && @new-pos[0] == @invalid[0] {
#             @choices := ((|@horizontal, |@vertical),);
#         } else {
#             @choices := ((|@vertical, |@horizontal), (|@horizontal, |@vertical));
#         }
#         @path.push(@choices);
#         @path.push(("A",));
#         @pos := @new-pos;
#     }
#     return @path;
# }

sub find-path-choices(%lookup, $start, $end) {
    # say "se $start $end {%lookup.raku}";
    my @start := %lookup{$start};
    my @end := %lookup{$end};
    # say "start end {@start.raku} :: {@end.raku}";
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