const Menu = enum {
    Bank,
    Market,
    Gov,
    User,
};

const Some = enum {
    Cash,
    Revenue,
    Expense,
    FR,
    BS,
    IS,
};

const Cash = enum {
    Type,
    Qty,
    Amt,
    Balance,
    Date,
};

const Revenue = enum {
    Seller,
    Name,
    Unit,
    Revenue,
    Quantity,
    UpdateAt,
};

const Expense = enum {
    Seller,
    Name,
    Unit,
    Expense,
    Quantity,
    UpdateAt,
};
