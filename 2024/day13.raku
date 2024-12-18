sub MAIN($input) {
    grammar Input {
        token TOP { <machine>+%"\n\n" "\n"* }
        token machine { <button-a> "\n" <button-b> "\n" <prize> }
        token button-a { "Button A: X+" (\d+) ", Y+" (\d+) }
        token button-b { "Button B: X+" (\d+) ", Y+" (\d+) }
        token prize { "Prize: X=" (\d+) ", Y=" (\d+) }
    }
    my $parsed = Input.parsefile($input);
    my $machines = $parsed<machine>.map({Map.new('a' => .<button-a>>>.Int.List, 'b' => .<button-b>>>.Int.List, 'prize' => .<prize>>>.Int.List)}).List;
    my $part1-solution = 0;
    for $machines.Seq -> $m {
        $part1-solution += fewest-tokens($m<a>, $m<b>, $m<prize>) || 0;
    }
    say "part1 solution: $part1-solution";

    my $part2-solution = 0;
    for $machines.Seq -> $m {
        $part2-solution += fewest-tokens($m<a>, $m<b>, $m<prize>.map(* + 10000000000000).List) || 0;
    }
    say "part2 solution: $part2-solution";
}

sub fewest-tokens($a-button, $b-button, $prize) {
    my $a-slope = [/] $a-button.reverse;
    my $b-slope = [/] $b-button.reverse;
    my $x = ($prize[1] - $b-slope * $prize[0]) / ($a-slope - $b-slope);
    my $a-presses = round($x / $a-button[0]);
    my $b-presses = round(($prize[0] - $x) / $b-button[0]);
    return Nil if (($a-button.map(* * $a-presses) Z+ $b-button.map(* * $b-presses)) Z== $prize.Seq).any == False;
    return $a-presses*3 + $b-presses;
}