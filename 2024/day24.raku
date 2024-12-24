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

    # z00   = XOR(x00, y00)
    # z00c  = AND(y00, x00)
    #
    # z01p  = XOR(x01, y01) # partial addition
    # z01   = XOR(z01p, z00c) # addition including carry-int
    # z01nc = AND(x01, y01) # normal x+y carry over
    # z01cc = AND(z01p, z00c) # x|y + c-in carry over
    # z01c  = OR(z01nc, z01cc) # carry-out. Only one branch can be true, so a XOR would work as well
    #
    # z02p  = XOR(x02, y02)
    # z02   = XOR(z02p, z001c)
    # z02nc = AND(x02, y02)
    # z02cc = AND(z02p, z01c)
    # z02c  = OR(z02nc, z02cc)
    my %inputs-to-gates;
    for %gates -> $gatekv {
        %inputs-to-gates{$gatekv.value[1].all}.push($gatekv.key);
    }
    # say %inputs-to-gates;

    my %named-gates;
    my %unexpected is SetHash;
    for 0..* {
        my $zgate = "z" ~ sprintf("%02d", $_);
        last if %gates{$zgate}:!exists;
        resolve-gate-symbols(%gates, %named-gates, $zgate, Nil, $_, %unexpected);
    }
    say %unexpected.keys.sort.join(",");

    # loop {
    #     my %wires = %initial-wires;
    #     for %wires.keys {
    #         # my $prefix = .substr(0, 1);
    #         # say "$_: ", !(so (.substr(0,1) eq ("x", "y").any));
    #         # say "$_: " <x y>.grep({$prefix == $_});
    #         next unless .substr(0,1) eq ("x", "y").any;
    #         %wires{$_} = rand > 0.5;
    #         # say "$_: {%wires{$_}}";
    #     }
    #     my ($x, $y, $z) = <x y z>.map({wires-to-int(%wires, %gates, $_)});
    #     say "x: ", sprintf("%x", $x);
    #     say "y: ", sprintf("%x", $y);
    #     say "expected z: ", sprintf("%x", $x + $y);
    #     say "actual   z: ", sprintf("%x", $z);
    #     exit;
    # }

    # for %gates.keys.sort {
    #     next if .substr(0,1) ne "z";
    #     say "$_: ", get-gate-expression(%gates, $_);
    # }
    say "part2-solution: $part2-solution";
}

# kinda worked, but depends on knowing the parent node for sure, which doesn't work
sub resolve-gate-symbols(%gates, %named-gates, $gate, $parent, $z-level, %unexpected) {
    # z00   = XOR(x00, y00)
    #
    # z00c  = AND(y00, x00)
    # z01p  = XOR(x01, y01) # partial addition
    # z01   = XOR(z01p, z00c) # addition including carry-int
    #
    # z01nc = AND(x01, y01) # normal x+y carry over
    # z01cc = AND(z01p, z00c) # x|y + c-in carry over
    # z01c  = OR(z01nc, z01cc) # carry-out. Only one branch can be true, so a XOR would work as well
    # say "gater: $gate";
    return $gate if $gate.substr(0,1) eq <x y>.any;
    return %named-gates{$gate} if %named-gates{$gate}:exists;

    my ($op, $input-wires) = %gates{$gate};
    my @input-symbols is List = $input-wires.map({resolve-gate-symbols(%gates, %named-gates, $_, $gate, $z-level, %unexpected)});
    return Nil unless @input-symbols.all.defined;
    @input-symbols := @input-symbols.sort.List;

    # say "Gate $gate:";
    # say "\t$op";
    # say "\t{@input-symbols}";
    if $gate eq "z00" {
        return Nil;
    }
    if $parent.defined && $parent eq "z01" {
        if $op eq "AND" {
            %named-gates{$gate} = "z00c";
            return "z00c";
        } elsif $op eq "XOR" {
            %named-gates{$gate} = "z01p";
            return "z01p";
        }
    }
    if $gate ~~ /^z(\d\d)$/ {
        # z00c  = AND(y00, x00)
        # z01p  = XOR(x01, y01) # partial addition
        # z01   = XOR(z01p, z00c) # addition including carry-int
        if $op ne "XOR" {
            say "$gate should be XOR, not $op";
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            return Nil;
        }
        if $parent.defined {
            say "$gate should not have a parent of $parent";
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            return Nil;
        }
        my $z-level = $/[0].Int;
        if @input-symbols !eqv (sprintf("z%02dc", $z-level -1), sprintf("z%02dp", $z-level)) {
            say "$gate has unexpected inputs: {@input-symbols}";
            say "expected {(sprintf("z%02dc", $z-level -1), sprintf("z%02dp", $z-level))}";
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            return Nil;
        }
        %named-gates{$gate} = $gate;
        return $gate;
    }
    if $parent ~~ /^z(\d\d)$/ {
        if $op eq "XOR" {
            if @input-symbols !eqv (sprintf("x%02d", $z-level), sprintf("y%02d", $z-level)) {
                say "$gate $op (parent: $parent) has unexpected inputs: {@input-symbols}";
                %named-gates{$gate} = Nil;
                %unexpected{$gate}++;
                return Nil;
            }
            %named-gates{$gate} = sprintf("z%02dp", $z-level);
            return sprintf("z%02dp", $z-level);
        } elsif $op eq "OR" {
            if @input-symbols !eqv (sprintf("z%02dcc", $z-level - 1), sprintf("z%02dnc", $z-level - 1)) {
                say "$gate (parent: $parent) has unexpected inputs: {@input-symbols}";
                %named-gates{$gate} = Nil;
                %unexpected{$gate}++;
                return Nil;
            }
            %named-gates{$gate} = sprintf("z%02dc", $z-level - 1);
            return sprintf("z%02dc", $z-level - 1);
        } else {
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            say "Not expecting $gate $op under $parent";
            say "\t{@input-symbols}";
            return Nil;
        }
    }
    # otherwise should be the carry-over ANDs
    if $op ne "AND" {
        say "expected $gate op:$op parent:$parent to be AND";
        say "\t{@input-symbols}";
        %named-gates{$gate} = Nil;
        %unexpected{$gate}++;
        return Nil;
    }
    if @input-symbols[0] ~~ /^x\d\d$/ && @input-symbols[1] ~~ /^y\d\d$/ {
        if @input-symbols[0].substr(1,2) ne @input-symbols[1].substr(1,2) {
            say "unexpecting combination of xy into AND gate $gate: {@input-symbols}";
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            return Nil;
        }
        %named-gates{$gate} = sprintf("z%snc", @input-symbols[0].substr(1,2));
        return sprintf("z%snc", @input-symbols[0].substr(1,2));
    }
    if @input-symbols[0] ~~ /^z\d\dc$/ && @input-symbols[1] ~~ /^z\d\dp$/ {
        my $zc-level = @input-symbols[0].substr(1,2).Int;
        my $zp-level = @input-symbols[1].substr(1,2).Int;
        if $zc-level + 1 != $zp-level {
            say "unexpected combination of zc and dp into AND gate $gate: {@input-symbols}";
            %named-gates{$gate} = Nil;
            %unexpected{$gate}++;
            return Nil;
        }
        %named-gates{$gate} = sprintf("z%scc", @input-symbols[1].substr(1,2));
        return sprintf("z%scc", @input-symbols[1].substr(1,2));
    }
    say "random input into AND gate $gate: {@input-symbols}";
    say "zc: ",@input-symbols[0] ~~ /^z\d\dc$/;
    say "zp: ",@input-symbols[1] ~~ /^z\d\dp$/;
    %named-gates{$gate} = Nil;
    %unexpected{$gate}++;
    return Nil;
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

sub create-full-adder($level) {
    return ("XOR", ("x00", "y00")) if $level == 0;
    return ("XOR", (("XOR", (pad0("x", $level), pad0("y", $level))), ))
}

sub pad0($c, $level) {
    sprintf("$c%02d", $level)
}

sub get-gate-expression(%gates, $gate) {
    return $gate if %gates{$gate}:!exists;
    my ($op, @input-wires) := %gates{$gate};
    my @expanded-inputs = @input-wires.map({get-gate-expression(%gates, $_)});
    return ($op, @expanded-inputs);
}

sub gate-expressions-equal($ge1, $ge2) {

}