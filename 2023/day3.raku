use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Line {
        token TOP { <tok>+ }
        token tok { <number> | <space> | <symbol> }
        token number { \d+ }
        token space { \. }
        token symbol { <-[\d\.]> }
    }

    my @numbers;
    my @symbols;
    my @grid;
    my $y = 0;
    for $file.lines -> $line {
        next unless $line; # ignore any empty lines
        for Line.parse($line)<tok> -> $token {
            my $id;
            given $token {
                when .<number> {
                    @numbers.push((+$y, $token.from, $_));
                    $id = ("num", @numbers.end);
                }
                when .<symbol> {
                    @symbols.push((+$y, $token.from, $_));
                    $id = ("sym", @symbols.end);
                }
                default {
                    $id = (".",)
                }
            }
            for $token.from..$token.to {
                @grid[$y;$_] = $id;
            }
        }
        $y += 1;
    }

    my $part-one-sum = 0;
    for @numbers -> ($y, $x, $number) {
        for ((-1..1)X(-1..$number.chars)).map({(.[0]+$y,.[1]+$x)}) -> ($i, $j) {
            next if $i < 0 || $j < 0 || (@grid[$i][$j]:!exists);
            if @grid[$i][$j][0] eq "sym" {
                $part-one-sum += +$number;
                last;
            }
        }
    }

    my $part-two-sum = 0;
    for @symbols -> ($y, $x, $symbol) {
        next unless $symbol eq "*";
        my %parts is SetHash;
        for ((-1..1)X(-1..1)).map({(.[0]+$y,.[1]+$x)}) -> ($i, $j) {
            next if $i < 0 || $j < 0 || (@grid[$i][$j]:!exists);
            my $id = @grid[$i][$j];
            if $id[0] eq "num" {
                %parts{$id[1]}++;
            }
        }
        if %parts.elems == 2 {
            $part-two-sum += [*] @numbers[%parts.keys][*;2];
        }
    }
    say "part 1: $part-one-sum";
    say "part 2: $part-two-sum";
}