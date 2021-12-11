const std = @import("std");

const Direction = enum {
    forward,
    up,
    down,
};

pub fn main() !void {
    const inputFile = try std.fs.cwd().openFile("input.txt", .{ .read = true });
    const input = inputFile.reader();
    var buf: [10]u8 = undefined;
    var horizontal_position: i32 = 0;
    var depth: i32 = 0;
    var aim: i32 = 0;
    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var line_iter = std.mem.split(u8, line, " ");
        const direction = std.meta.stringToEnum(Direction, line_iter.next().?).?;
        const amount = try std.fmt.parseInt(i32, line_iter.rest(), 10);
        //std.log.info("input: direction: {}, amount: {}", .{direction, amount});
        switch (direction) {
            .forward => {
                horizontal_position += amount;
                depth += aim * amount;
            },
            .up => aim -= amount,
            .down => aim += amount,
        }
    }
    std.log.info("final position: horizontal position {} x depth {} = {}", .{ horizontal_position, depth, horizontal_position * depth });
}
