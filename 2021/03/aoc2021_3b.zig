const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const input_file = try std.fs.cwd().openFile("input.txt", .{ .read = true });
    const input = input_file.reader();
    var buf: [12]u8 = undefined;

    var num_lines: usize = 0;
    var line_length: usize = 0;

    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        line_length = line.len;
        num_lines += 1;
    }

    var values = try allocator.alloc(i32, num_lines);
    try readAllDiagnostics(&input_file, values);

    var gamma_depth: u5 = 0;
    var gamma_values = values[0..];
    while (true) {
        const counters = try doCount(allocator, gamma_values, line_length);
        const gamma = getGamma(counters, gamma_values.len);
        const gamma_digit = binAt(gamma, gamma_depth, line_length);
        //std.log.info("gamma: {b}, gamma_digit: {}", .{gamma, gamma_digit});
        var i: usize = 0;
        for (gamma_values) |value| {
            const digit = binAt(value, gamma_depth, line_length);
            if (digit != gamma_digit) {
                continue;
            }
            gamma_values[i] = value;
            i += 1;
        }
        gamma_values = gamma_values[0..i];
        //std.log.info("gamma: {}, gamma_values len: {}", .{ gamma_depth, gamma_values.len });
        //std.log.info("gamma_values: {b}", .{ gamma_values });
        gamma_depth += 1;
        if (gamma_values.len <= 1) {
            break;
        }
    }
    const oxygen = gamma_values[0];

    try readAllDiagnostics(&input_file, values);
    var epsilon_depth: u5 = 0;
    var epsilon_values = values[0..];
    while (true) {
        const counters = try doCount(allocator, epsilon_values, line_length);
        const epsilon = getEpsilon(counters, epsilon_values.len);
        const epsilon_digit = binAt(epsilon, epsilon_depth, line_length);
        //std.log.info("epsilon: {b}, epsilon_digit: {}", .{epsilon, epsilon_digit});
        var i: usize = 0;
        for (epsilon_values) |value| {
            const digit = binAt(value, epsilon_depth, line_length);
            if (digit != epsilon_digit) {
                continue;
            }
            epsilon_values[i] = value;
            i += 1;
        }
        epsilon_values = epsilon_values[0..i];
        //std.log.info("epsilon: {}, epsilon_values len: {}", .{ epsilon_depth, epsilon_values.len });
        //std.log.info("epsilon_values: {b}", .{ epsilon_values });
        epsilon_depth += 1;
        if (epsilon_values.len <= 1) {
            break;
        }
    }
    const co2 = epsilon_values[0];
    std.log.info("oxygen generator rating: {}, CO2 scrubber rating: {}, life support rating: {}", .{ oxygen, co2, oxygen * co2 });
}

fn getGamma(counters: []i32, num_lines: usize) i32 {
    var gamma: i32 = 0;
    for (counters) |digit| {
        //std.log.info("counter digit: {}, {}", .{ digit, num_lines });
        var gamma_digit: i32 = if (digit >= (num_lines - @intCast(usize, digit))) 1 else 0;
        gamma = (gamma << 1) | gamma_digit;
    }
    return gamma;
}
fn getEpsilon(counters: []i32, num_lines: usize) i32 {
    var epsilon: i32 = 0;
    for (counters) |digit| {
        //std.log.info("counter digit: {}, {}", .{ digit, num_lines });
        var epsilon_digit: i32 = if (digit >= (num_lines - @intCast(usize, digit))) 0 else 1;
        epsilon = (epsilon << 1) | epsilon_digit;
    }
    return epsilon;
}

fn doCount(allocator: std.mem.Allocator, values: []i32, line_length: usize) ![]i32 {
    const counters: []i32 = try allocator.alloc(i32, line_length);
    for (counters) |*value| {
        value.* = 0;
    }

    for (values) |value| {
        //std.log.info("value: {b}", .{value});
        for (counters) |*counter, i| {
            counter.* += binAt(value, i, line_length);
        }
    }
    return counters;
}

fn readAllDiagnostics(input_file: *const std.fs.File, values: []i32) !void {
    try input_file.seekTo(0);
    const input = input_file.reader();
    var buf: [12]u8 = undefined;
    var i: usize = 0;
    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const value = try std.fmt.parseInt(i32, line, 2);
        values[i] = value;
        i += 1;
    }
}

fn binAt(number: i32, offset: usize, line_length: usize) i32 {
    return (number >> @intCast(u5, (line_length - 1 - offset))) & 1;
}
