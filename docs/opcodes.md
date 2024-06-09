# Grape Virtual Machine Opcodes

This file provides an overview of the operation codes (opcodes) supported by the Grape Virtual Machine.

| opcode   | form                        | operands | description              |
| -------- | --------------------------- | -------- | ------------------------ |
| HALT     | 0x0                         |          | Abrupt stop              |
| RETURN   | 0x1                         |          | Return from call         |
| ICONST_0 | 0x2                         |          | Push integer 0 constant  |
| ICONST_1 | 0x3                         |          | Push integer 1 constant  |
| LOAD     | 0x4, index                  |          | Load from local variable |
| STORE    | 0x5, index                  |          | Store to local variable  |
| FCONST_0 | 0x6                         |          | Push float 0 constant    |
| FCONST_1 | 0x7                         |          | Push float 1 constant    |
| LOAD_0   | 0x8                         |          | Load from local variable 0 |
| LOAD_1   | 0x9                         |          | Load from local variable 1 |
| LOAD_2   | 0xA                         |          | Load from local variable 2 |
| LOAD_3   | 0xB                         |          | Load from local variable 3 |
| I2F      | 0xC                         |          | Convert integer to float |
| F2I      | 0xD                         |          | Convert float to integer |
| GOTO     | 0xE, index1, index2         |          | Always branch, u16 index |
| CALL     | 0xF, mod_index1, mod_index2, fun_index1, fun_index2 | args... -> | Call function, u16 indexes, module and function should point to a valid Module/Function entry in the constant pool |
| LOADCONST | 0x10, index         |          | Load and push item from constant pool |
| NEW_DICT | 0x11               |          | Create new dict, push a reference to the stack |
| SET_DICT  | 0x12               | ref, field, value -> | Set a value in the dict field  |
| GET_DICT  | 0x13               | ref, field -> value  | Get value from dict field  |
| I_PUSH_BYTE  | 0x14, byte           |        | Push 1 byte long integer |
| I_PUSH_SHORT | 0x15, short1, short2 |        | Push 2 byte long integer |
| POP          | 0x16                 |        | Pop 1 value from stack |
| I_IFEQ  | 0x17, index1, index2 | value1, value2 -> | Branch if integer is equal |
| I_IFNEQ | 0x18, index1, index2 | value1, value2 -> | Branch if integer is not equal |
| I_IFGT | 0x19, index1, index2 | value1, value2 -> | Branch if integer is greater than |
| I_IFGE | 0x1A, index1, index2 | value1, value2 -> | Branch if integer is greater or equal |
| I_IFLT | 0x1B, index1, index2 | value1, value2 -> | Branch if integer is less than |
| I_IFLE | 0x1C, index1, index2 | value1, value2 -> | Branch if integer is less or equal |
| IADD  | 0x1D | value1, value2 -> result | Add integer |
| ISUB  | 0x1E | value1, value2 -> result | Subtract integer |
| IMUL  | 0x1F | value1, value2 -> result | Multiply integer |
| IDIV  | 0x20 | value1, value2 -> result | Divide integer |
| IREM  | 0x21 | value1, value2 -> result | Remainder of integer |
| IAND  | 0x22 | value1, value2 -> result | Integer bit AND |
| IOR   | 0x23 | value1, value2 -> result | Integer bit OR |
| IXOR  | 0x24 | value1, value2 -> result | Integer bit XOR |
| ISHL  | 0x25 | value1, value2 -> result | Integer bit shift left |
| ISHR  | 0x26 | value1, value2 -> result | Integer bit shift right |
| IUSHR | 0x27 | value1, value2 -> result | Integer logical bit shift right |
| INEG  | 0x28 | value -> result          | Negate integer |
| STORE_0 | 0x29 |     | Store to local variable 0 |
| STORE_1 | 0x2A |     | Store to local variable 1 |
| STORE_2 | 0x2B |     | Store to local variable 2 |
| STORE_3 | 0x2C |     | Store to local variable 3 |
| DUP     | 0x2D | value -> value, value | Duplicate from stack |
| ~       | 0x2E |         | Reserved, not implemented |
| NEW_ARRAY | 0x2F | size -> ref | Allocate new sized array |
| ARRAY_GET | 0x30 | ref, index -> ref | Get index from array |
| ARRAY_SET | 0x31 | ref, index, value -> | Set index to array |
| IINC      | 0x32, index, inc |  | 1 Byte local variable increment |
| IF_NULL    | 0x33, index1, index2 | value -> | Branch if null |
| IFNOT_NULL | 0x34, index1, index2 | value -> | Branch if not null |
| CONST_NULL | 0x35                 |          | Push null constant |
| IEXP       | 0x36 | value1, value2 -> result | Integer exponent |
| IS_ZERO    | 0x37 | value | Push integer 1 if value is zero |
| TAILCALL   | 0x38 | args... -> | Tailcall the current function |
| FADD | 0x39 | value1, value2 -> result | Add float |
| FSUB | 0x3A | value1, value2 -> result | Subtract float |
| FMUL | 0x3B | value1, value2 -> result | Multiply float |
| FDIV | 0x3C | value1, value2 -> result | Divide float |
| FREM | 0x3D | value1, value2 -> result | Remainder of float |
| FNEG | 0x3E | value -> result          | Negate float |
| PUSH_BYTE | 0x3F, byte | | Push byte literal |
| BADD | 0x40 | value1, value2 -> result | Add byte |
| BSUB | 0x41 | value1, value2 -> result | Subtract byte |
| BMUL | 0x42 | value1, value2 -> result | Multiply byte |
| BDIV | 0x43 | value1, value2 -> result | Divide byte |
| BREM | 0x44 | value1, value2 -> result | Remainder of byte |
| BAND | 0x45 | value1, value2 -> result | Byte bit AND |
| BOR  | 0x46 | value1, value2 -> result | Byte bit OR |
| BXOR | 0x47 | value1, value2 -> result | Byte bit XOR |
| BSHL | 0x48 | value1, value2 -> result | Byte bit shift left |
| BSHR | 0x49 | value1, value2 -> result | Byte bit shift right |
| BNEG | 0x4A | value -> result          | Negate byte |
| NEW_BYTES | 0x4B, len1, len2 | bytes... -> | Create bytes object |
| BYTES_PUSH | 0x4C | ref, byte -> | Push byte to bytes object |
| NEW         | 0x4D, class1, class2 | args... ->      | Create new class object |
| CALL_METHOD | 0x4E, class1, class2, method1, method2 | ref, args... -> | Call method from class object |
| SET_FIELD   | 0x4F, field1, field2 | ref, value ->   | Set field to class object |
| GET_FIELD   | 0x50, field1, field2 | ref -> | Get field from class object |
