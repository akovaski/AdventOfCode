sub MAIN($input) {
    my $file = open $input;
    my @stones = $file.slurp.trim.words>>.Int;

    my $stone-bag = bag @stones;
    for 0..^25 {
        $stone-bag = blink($stone-bag);
    }
    my $part1-solution = $stone-bag.values.sum;
    say "part 1: $part1-solution";

    for 25..^75 {
        $stone-bag = blink($stone-bag);
    }
    my $part2-solution = $stone-bag.values.sum;
    say "part 2: $part2-solution";
}

sub blink($stone-bag) {
    my BagHash $new-bag;
    for $stone-bag.kv -> $stone, $count {
        for evolve-stone($stone) {
            $new-bag{$_} += $count;
        }
    }
    return $new-bag;
}

sub evolve-stone($stone) {
    return 1 if $stone == 0;

    my $stone-word = $stone.Str;
    if $stone-word.chars %% 2 {
        return ($stone-word.substr(0, * div 2), $stone-word.substr(* div 2, *))>>.Int;
    }

    return 2024 * $stone;
}