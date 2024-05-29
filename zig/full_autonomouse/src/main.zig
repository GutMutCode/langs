const print = std.debug.print;
const log = std.log.info;
const std = @import("std");
const expect = std.testing.expect;
const assert = std.debug.assert;

pub fn main() void { // don't need to write '!'
    std_print();
}
/// Standard library & Error Union type
fn std_tst() !void {
    // constant identifier
    const stdout = std.io.getStdOut().writer();
    // stdout.print("Hello, world!\n", .{}) catch |err| return err; // this is equivalent to
    try stdout.print("Hello, world!\n", .{}); // this may be crashed

    // another print functions
    print("Hello, Debug!\n", .{});
    log("Hello, Log!\n", .{});
}

fn std_print() void {
    const one_plus_one: i32 = 1 + 1;
    assert(one_plus_one == 2);
    print("Assert is Correct! {}\n", .{one_plus_one});

    var optional_value: ?[]const u8 = null;
    assert(optional_value == null);

    // you need to indicate there is an optional value by using ?
    print("\noptional 1\ntype: {}\nvalue: {?s}\n", .{ @TypeOf(optional_value), optional_value });

    optional_value = "hi";
    assert(optional_value != null);

    print("\noptional 2\ntype: {}\nvalue: {?s}\n", .{ @TypeOf(optional_value), optional_value });

    // create error union
    var number_or_error: anyerror!i32 = error.ArgumentError;

    // you need to indicate the value is can be an error union by using ?
    print("\nerror union 1\ntype: {}\nvalue: {!}\n", .{ @TypeOf(number_or_error), number_or_error });

    number_or_error = 1234;

    print("\nerror union 2\ntype: {}\nvalue: {!}\n", .{ @TypeOf(number_or_error), number_or_error });
}
