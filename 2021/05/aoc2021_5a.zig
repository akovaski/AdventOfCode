const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    const input_file_name = if (args.len >= 2) args[1] else "input.txt";

    const input_file = try std.fs.cwd().openFile(input_file_name, .{ .read = true });
    const input = input_file.reader();
    var buf: [300]u8 = undefined;
    var grid: Grid = undefined;
    for (grid) |*row| {
        for (row) |*val| {
            val.* = 0;
        }
    }

    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var endpoints = std.mem.split(u8, line, " -> ");
        const a = try parsePoint(endpoints.next().?);
        const b = try parsePoint(endpoints.next().?);
        const offset = a.diff(b);
        if (offset.x != 0 and offset.y != 0) {
            // ignore diagonals for now
            continue;
        }
        drawLine(&grid, a, b);
        //std.log.info("a: {}, b: {}, offset: {}", .{a, b, offset});
    }

    var num_dangerous: i64 = 0;
    for (grid) |*row, i| {
        if (i < 10) {
            //std.log.info("grid: {d}", .{row[0..10].*});
        }
        for (row) |val| {
            if (val >= 2) {
                num_dangerous += 1;
            }
        }
    }
    std.log.info("Dangerous areas: {}", .{num_dangerous});
}

const Grid = [1000][1000]u16;

fn drawLine(grid: *Grid, a: Point, b: Point) void {
    var pi = PointIterator.line(a, b);
    while (pi.next()) |point| {
        point.draw(grid);
    }
}

const Point = struct {
    x: i16,
    y: i16,

    fn diff(a: Point, b: Point) PointOffset {
        return PointOffset{
            .x = b.x - a.x,
            .y = b.y - a.y,
        };
    }

    fn add(a: Point, b: PointOffset) Point {
        return Point{
            .x = a.x + b.x,
            .y = a.y + b.y,
        };
    }

    fn draw(self: Point, grid: *Grid) void {
        grid[@intCast(usize, self.y)][@intCast(usize, self.x)] += 1;
    }
};

const PointIterator = struct {
    current: Point,
    end: Point,
    finished: bool = false,

    fn line(a: Point, b: Point) PointIterator {
        return PointIterator{
            .current = a,
            .end = b,
        };
    }

    fn next(self: *PointIterator) ?Point {
        if (self.finished) {
            return null;
        }
        const unit_offset = self.current.diff(self.end).unit();
        const ret = self.current;
        self.current = self.current.add(unit_offset);
        if (unit_offset.x == 0 and unit_offset.y == 0) {
            self.finished = true;
        }
        return ret;
    }
};

const PointOffset = struct {
    x: i16,
    y: i16,

    fn unit(self: PointOffset) PointOffset {
        return PointOffset{
            .x = intUnit(i16, self.x),
            .y = intUnit(i16, self.y),
        };
    }
};

fn intUnit(comptime T: type, int: T) T {
    if (int > 0) {
        return 1;
    } else if (int < 0) {
        return -1;
    } else {
        return 0;
    }
}

fn parsePoint(s: []const u8) !Point {
    var coordinates = std.mem.split(u8, s, ",");
    const x = try std.fmt.parseInt(i16, coordinates.next().?, 10);
    const y = try std.fmt.parseInt(i16, coordinates.next().?, 10);
    return Point{ .x = x, .y = y };
}
