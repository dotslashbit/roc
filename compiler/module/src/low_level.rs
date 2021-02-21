/// Low-level operations that get translated directly into e.g. LLVM instructions.
/// These are always wrapped when exposed to end users, and can only make it
/// into an Expr when added directly by can::builtins
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LowLevel {
    StrConcat,
    StrJoinWith,
    StrIsEmpty,
    StrStartsWith,
    StrEndsWith,
    StrSplit,
    StrCountGraphemes,
    StrFromInt,
    StrFromUtf8,
    StrFromFloat,
    ListLen,
    ListGetUnsafe,
    ListSet,
    ListSetInPlace,
    ListSingle,
    ListRepeat,
    ListReverse,
    ListConcat,
    ListContains,
    ListAppend,
    ListPrepend,
    ListJoin,
    ListMap,
    ListMapWithIndex,
    ListKeepIf,
    ListWalk,
    ListWalkBackwards,
    ListSum,
    ListKeepOks,
    ListKeepErrs,
    DictSize,
    DictEmpty,
    DictInsert,
    DictRemove,
    DictContains,
    DictGetUnsafe,
    DictKeys,
    DictValues,
    DictUnion,
    DictIntersection,
    DictDifference,
    DictWalk,
    SetFromList,
    NumAdd,
    NumAddWrap,
    NumAddChecked,
    NumSub,
    NumSubWrap,
    NumSubChecked,
    NumMul,
    NumMulWrap,
    NumMulChecked,
    NumGt,
    NumGte,
    NumLt,
    NumLte,
    NumCompare,
    NumDivUnchecked,
    NumRemUnchecked,
    NumAbs,
    NumNeg,
    NumSin,
    NumCos,
    NumSqrtUnchecked,
    NumRound,
    NumToFloat,
    NumPow,
    NumCeiling,
    NumPowInt,
    NumFloor,
    NumIsFinite,
    NumAtan,
    NumAcos,
    NumAsin,
    NumBitwiseAnd,
    NumBitwiseXor,
    NumBitwiseOr,
    NumShiftLeftBy,
    NumShiftRightBy,
    NumShiftRightZfBy,
    Eq,
    NotEq,
    And,
    Or,
    Not,
    Hash,
}
