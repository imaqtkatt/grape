/// Abrupt stop.
pub const HALT: u8 = 0x0;

/// Return from call.
pub const RETURN: u8 = 0x1;

/// Push integer 0 constant.
pub const ICONST_0: u8 = 0x2;

/// Push integer 1 constant.
pub const ICONST_1: u8 = 0x3;

/// Load from local variable.
pub const LOAD: u8 = 0x4;

/// Store to local variable.
pub const STORE: u8 = 0x5;

/// Push float 0 constant.
pub const FCONST_0: u8 = 0x6;

// Push float 1 constant.
pub const FCONST_1: u8 = 0x7;

/// Load local variable 0.
pub const LOAD_0: u8 = 0x8;

/// Load local variable 1.
pub const LOAD_1: u8 = 0x9;

/// Load local variable 2.
pub const LOAD_2: u8 = 0xA;

/// Load local variable 3.
pub const LOAD_3: u8 = 0xB;

/// Convert integer to float.
pub const I2F: u8 = 0xC;

/// Convert float to integer.
pub const F2I: u8 = 0xD;

/// Always branch, u8 index.
pub const GOTO: u8 = 0xE;

/// Call function.
pub const CALL: u8 = 0xF;

/// Push item from constant pool.
pub const LOADCONST: u8 = 0x10;

/// Create new object, push reference.
pub const NEW_OBJECT: u8 = 0x11;

/// Set object field.
pub const SET_FIELD: u8 = 0x12;

/// Get object field.
pub const GET_FIELD: u8 = 0x13;

/// Push 1 byte long integer.
pub const PUSH_BYTE: u8 = 0x14;

/// Push 2 bytes long integer.
pub const PUSH_SHORT: u8 = 0x15;

/// Pop from stack.
pub const POP: u8 = 0x16;

/// Branch if integer is equal.
pub const I_IFEQ: u8 = 0x17;

/// Branch if integer is not equal.
pub const I_IFNEQ: u8 = 0x18;

/// Branch if integer is greater than.
pub const I_IFGT: u8 = 0x19;

/// Branch if integer is greater or equal.
pub const I_IFGE: u8 = 0x1A;

/// Branch if integer is less than.
pub const I_IFLT: u8 = 0x1B;

/// Branch if integer is less or equal.
pub const I_IFLE: u8 = 0x1C;

/// Add integer.
pub const IADD: u8 = 0x1D;

/// Subtract integer.
pub const ISUB: u8 = 0x1E;

/// Multiply integer.
pub const IMUL: u8 = 0x1F;

/// Divide integer.
pub const IDIV: u8 = 0x20;

/// Remainder of integer.
pub const IREM: u8 = 0x21;

/// Integer bit AND.
pub const IAND: u8 = 0x22;

/// Integer bit OR.
pub const IOR: u8 = 0x23;

/// Integer bit XOR.
pub const IXOR: u8 = 0x24;

/// Integer bit shift left.
pub const ISHL: u8 = 0x25;

/// Integer bit shift right.
pub const ISHR: u8 = 0x26;

/// Integer logical bit shift right.
pub const IUSHR: u8 = 0x27;

/// Negate integer.
pub const INEG: u8 = 0x28;

/// Store to local variable 0.
pub const STORE_0: u8 = 0x29;

/// Store to local variable 1.
pub const STORE_1: u8 = 0x2A;

/// Store to local variable 2.
pub const STORE_2: u8 = 0x2B;

/// Store to local variable 3.
pub const STORE_3: u8 = 0x2C;

/// Duplicate from stack.
pub const DUP: u8 = 0x2D;

/// Allocate new string.
pub const NEW_STRING: u8 = 0x2E;

/// Allocate new array.
pub const NEW_ARRAY: u8 = 0x2F;

/// Get element from array by index.
pub const ARRAY_GET: u8 = 0x30;

/// Set element from array by index.
pub const ARRAY_SET: u8 = 0x31;

/// Increment local variable by value.
pub const IINC: u8 = 0x32;

/// Branch if null.
pub const IF_NULL: u8 = 0x33;

/// Branch if not null.
pub const IFNOT_NULL: u8 = 0x34;

/// Push null constant to stack.
pub const CONST_NULL: u8 = 0x35;

/// Exponent from integer.
pub const IEXP: u8 = 0x36;

/// Push 0 or 1 if it is zero.
pub const IS_ZERO: u8 = 0x37;

pub const TAILCALL: u8 = 0x38;

/// Add float.
pub const FADD: u8 = 0x39;

/// Subtract float.
pub const FSUB: u8 = 0x40;

/// Multiply float.
pub const FMUL: u8 = 0x41;

/// Divide float.
pub const FDIV: u8 = 0x42;

/// Remainder of float.
pub const FREM: u8 = 0x43;

/// Negate float.
pub const FNEG: u8 = 0x44;

/// Opcode repr table.
pub const TO_STR: &[&str] = &[
  "RET",
  "RETURN",
  "ICONST_0",
  "ICONST_1",
  "LOAD",
  "STORE",
  "FCONST_0",
  "FCONST_1",
  "LOAD_0",
  "LOAD_1",
  "LOAD_2",
  "LOAD_3",
  "I2F",
  "F2I",
  "GOTO",
  "CALL",
  "LOADCONST",
  "NEW_OBJECT",
  "SET_FIELD",
  "GET_FIELD",
  "PUSH_BYTE",
  "PUSH_SHORT",
  "POP",
  "IFEQ",
  "IFNEQ",
  "IFGT",
  "IFGE",
  "IFLT",
  "IFLE",
  "IADD",
  "ISUB",
  "IMUL",
  "IDIV",
  "IREM",
  "IAND",
  "IOR",
  "IXOR",
  "ISHL",
  "ISHR",
  "IUSHR",
  "INEG",
  "STORE_0",
  "STORE_1",
  "STORE_2",
  "STORE_3",
  "DUP",
  "NEW_STRING",
  "NEW_ARRAY",
  "ARRAY_GET",
  "ARRAY_SET",
  "IINC",
  "IF_NULL",
  "IFNOT_NULL",
  "CONST_NULL",
  "IEXP",
  "ISZERO",
  "TAILCALL",
  "FADD",
  "FSUB",
  "FMUL",
  "FDIV",
  "FREM",
  "FNEG",
];
