# Sheet

```rust
struct Sheet {
  id: i32,
  name: String
}
```

The `Sheet` type represents a calculus sheet. The sheet has a name and an unique id. It is a type that can be used to represent all type of budgets, a monthly budget or a weekly budget or even a budget for a specific event. The type is vague for this specific reason.

___

```rust
struct Expense {
  id: i32,
  name: String,
  amount: i32
}

struct Income {
  id: i32,
  name: String,
  amount: i32
}
```

A `Sheet` is then composed of sources of incomes (`Income`) and expenses (`Expense`). You get the final budget for the sheet by adding all the `Income`s and substracting the `Expense`s.

___

```rust
struct InheritedSheet {
  sheet_id: i32,
  inherit_incomes: bool,
  inherit_expenses: bool
}
```

But a sheet is not only composed of these two types, a sheet can also be composed of other sheets. Inheriting of the expenses and the incomes if wanted. The inheritance is optional and can be chosen every time you include an other `Sheet`

___

calculating a sheet's budget can be a long process, running through multiple sheets recursively. For this reason we must be careful when inheriting from sheets, checking for infinite loops.

```rust
struct CachedSheetBudget {
  id: i32,
  sheet_id: i32,
  amount: i32
}
```

We can also use a caching table to avoid recalculating a sheet's budget everytime we need its value. The cached value should be updated everytime one of the sheet' values changes, is added or is removed. This means updating a simple row in the `Expense` table can create changes in multiple sheets, as the changes propagate from the bottom sheet to the top sheets who inherits from the first ones.

Doing this creates faster read-access but slower write-access.

___

```rust
struct PendingSheetUpdate {
  sheet_id: i32
}
```

to avoid a stack overflow when updating long sheet inheritance trees, the updating logic must not use recursive code. To achieve this, we use a queue where a pending update is pushed everytime we encounter one. We run through each sheet in the queue, updating its value, then adding into the queue every other sheet inheriting from the current sheet until there is no sheet in the queue anymore.

___

```rust
const USE_CACHED_SHEET_VALUES: bool;
```

It can be useful to be able to change the way the system works around the cached values with a single boolean. Its default value should be `true`.

Once set to `false`, the system does not use the sheets cached values. It gains in write and update speed. This is useful for workflows where lots of change are required.

Once set to `true`, the system does use the sheets cached values. It gains in read speed but loses in write and update speed. This is useful for worfklows where lots of sheets are used, and inheritence is used frequently.