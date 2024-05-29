//! A set of structs
const std = @import("std");
const Vector = std.meta.Vector;

/// Finantial Result
const FR = struct {
    Credit: f64,
    AssetTurnover: f64,
    StorageTurnover: f64,
    InventoryTurnover: f64,
    DaylySalesInInventory: f64,
    GrossMargin: f64,
};

const BS = struct {
    USD: f64,
    KRW: f64,
    /// MarketableSecurities
    MS: f64,
    Inventory: f64,
    Storage: f64,
    CurrentAssets: f64,
};

const IS = struct {
    Revenue: f64,
    OtherExpense: f64,
    SalesDiscount: f64,
    CostOfGoodsSold: f64,
    CompensationExpense: f64,
    /// DepreciationAndAmortizationExpense
    DAAExpense: f64,
};

const Tax = struct {
    OrderID: Vector(u8),
    TaxpayerID: Vector(u8),
};

pub const Point = struct {
    x: f32,
    y: f32,

    pub fn init(x: f32, y: f32) Point {
        return Point{ .x = x, .y = y };
    }
};

pub const Msg = struct {
    msg: []const u8,
    who: []const u8,
};

const hello = Msg{ .msg = "hello", .who = "world" };

test "say_hello" {
    try std.testing.expectEqualStrings("hello", hello.msg);
}
