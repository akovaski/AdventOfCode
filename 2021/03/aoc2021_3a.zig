const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const inputFile = try std.fs.cwd().openFile("input.txt", .{ .read = true });
    const input = inputFile.reader();
    var buf: [12]u8 = undefined;

    var num_lines: i32 = 0;
    var counters: ?[]i32 = null;

    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        if (counters == null) {
            counters = try allocator.alloc(i32, line.len);
            for (counters.?) |*value| {
                value.* = 0;
            }
        }
        for (line) |digit, i| {
            switch (digit) {
                '0' => {},
                '1' => counters.?[i] += 1,
                else => unreachable,
            }
        }
        //std.log.info("input: {s}, {any}, {}", .{line, counters, num_lines});
        num_lines += 1;
    }

    var gamma: i32 = 0;
    var mask: i32 = 0;
    for (counters.?) |digit| {
        //std.log.info("counter digit: {}, {}", .{ digit, num_lines - digit });
        var gamma_digit: i32 = if (digit > (num_lines - digit)) 1 else 0;
        gamma = (gamma << 1) | gamma_digit;
        mask = (mask << 1) | 1;
    }
    const epsilon = (~gamma) & mask;
    const power_consumption = gamma * epsilon;
    std.log.info("gamma: {}, epsilon: {}, power consumption: {}", .{ gamma, epsilon, power_consumption });
}
