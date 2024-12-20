sub MAIN($input) {
    grammar Input {
        token TOP { <register-a> "\n" <register-b> "\n" <register-c> "\n\n" <program> "\n"* }
        token register-a { "Register A: " (\d+) }
        token register-b { "Register B: " (\d+) }
        token register-c { "Register C: " (\d+) }
        token program { "Program: " (\d)+%"," }
    }
    my $parsed = Input.parsefile($input);
    my $register-a = $parsed<register-a>[0].Int;
    my $register-b = $parsed<register-b>[0].Int;
    my $register-c = $parsed<register-c>[0].Int;
    my @program = $parsed<program>[0]>>.Int;

    my $part1-solution = run-program($register-a, $register-b, $register-c, @program).join(",");
    say "part1 solution: $part1-solution";

    my $part2-solution = search-for-quine(0, $register-b, $register-c, @program, 0);
    say "part2-solution: $part2-solution";
}

sub run-program($a, $b, $c, @program) {
    my ($register-a, $register-b, $register-c) Z= ($a, $b, $c);
    my $pc = 0;
    my @output;
    while $pc < @program.elems {
        my ($opcode, $operand) Z= @program[$pc, $pc+1];
        my $combo = (given $operand {
            when 0..3 { $operand }
            when 4 { $register-a }
            when 5 { $register-b }
            when 6 { $register-c }
            when 7 { Nil }
            default { say "invalid operand $operand"; exit; }
        });
        given $opcode {
            when 0 { $register-a = $register-a div (2 ** $combo); }
            when 1 { $register-b = $register-b +^ $operand; }
            when 2 { $register-b = $combo mod 8; }
            when 3 { $pc = $operand - 2 if $register-a != 0; }
            when 4 { $register-b = $register-b +^ $register-c; }
            when 5 { @output.push($combo mod 8); }
            when 6 { $register-b = $register-a div (2 ** $combo); }
            when 7 { $register-c = $register-a div (2 ** $combo); }
            default { say "invalid opcode $opcode"; exit; }
        }
        $pc += 2;
    }
    return @output;
}

sub search-for-quine($a, $b, $c, @program, $idx) {
    return $a if $idx == @program.elems;
    for ^8 {
        my $test-solution = $a * 8 + $_;
        my @output = run-program($test-solution, $b, $c, @program);
        my @program-slice = @program[*-1-$idx..*];
        if @program-slice eqv @output {
            my $found = search-for-quine($test-solution, $b, $c, @program, $idx+1);
            if $found {
                return $found;
            }
        }
    }
    # Time to back track...
    return False;
}