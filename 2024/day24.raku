sub MAIN($input) {
    grammar Input {
        token TOP { <wire>+%"\n" "\n\n" <gate>+%"\n" "\n"* }
        token wire { (\w+) ": " (\d) }
        token gate { (\w+) " " (\w+) " " (\w+) " -> " (\w+) }
    }
    my $parsed = Input.parsefile($input);
    my %initial-wires = $parsed<wire>.map({.[0].Str => .[1].Int.Bool});
    my %gates = $parsed<gate>.map({.[3].Str => (.[1].Str, (.[0].Str, .[2].Str))});
    # say %initial-wires.raku;
    # say %gates.raku;

    my $part1-solution = wires-to-int(%initial-wires, %gates, "z");
    say "part1 solution: $part1-solution";

    my $part2-solution = 0;
    say "part2-solution: $part2-solution";
}

sub wires-to-int(%wires, %gates, $prefix) {
    my $z = 0;
    if $prefix eq <x y>.any {
        for %wires.keys {
            next if .substr(0,1) ne $prefix;
            $z += 1 +< .substr(1,*).Int if %wires{$_};
        }
    } else {
        for %gates.keys {
            next if .substr(0,1) ne $prefix;
            resolve-gates(%wires, %gates, $_);
            $z += 1 +< .substr(1,*).Int if %wires{$_};
        }
    }
    return $z;
}

sub resolve-gates(%wires, %gates, $gate) {
    my ($op, @input-wires) := %gates{$gate};
    my @input-values = @input-wires.map({resolve-gates(%wires, %gates, $_) if %wires{$_}:!exists; %wires{$_}});
    %wires{$gate} = (given $op {
        when "AND" { [&&] @input-values }
        when "XOR" { [!=] @input-values }
        when "OR"  { [||] @input-values }
    });
    # say "$gate = $op ({@input-values.raku}) = {%wires{$gate}}";
}
