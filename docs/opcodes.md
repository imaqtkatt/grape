# Grape Virtual Machine Opcodes

This documentation provides a overview of the operation codes (opcodes) supported by the Grape Virtual Machine, including specifics behaviors for each opcode.

## Arithmetic Operations

### `IADD`

- **Description**: Adds the top two integer values from the stack.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `ISUB`

- **Description**: Subtracts the top stack value from the second-top stack value.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IMUL`

- **Description**: Multiplies the top two stack values.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IDIV`

- **Description**: Divides the second-top stack value by the top stack value.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IREM`

- **Description**: Computes the remainder of the division of the second-top stack value by the top stack value.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `INEG`

- **Description**: Negates the top integer on the stack.
- **Stack Behavior**: Pops 1 value, pushes the result (1 value).

## Logical Operations

### `IAND`

- **Description**: Performs a bitwise AND on the top two stack values.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IOR`

- **Description**: Performs a bitwise OR on the top two stack values.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IXOR`

- **Description**: Performs a bitwise XOR on the top two stack values.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `ISHL`

- **Description**: Shifts the second-top stack value left by the number of bits specified by the top stack value.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `ISHR`

- **Description**: Shifts the second-top stack value right by the number of bits specified by the top stack value, preserving the sign.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

### `IUSHR`

- **Description**: Shifts the second-top stack value right (unsigned) by the number of bits specified by the top stack value.
- **Stack Behavior**: Pops 2 values, pushes the result (1 value).

## Stack Manipulation

### `DUP`

- **Description**: Duplicates the top value on the stack.
- **Stack Behavior**: Pops 0 values, pushes 1 duplicate value.

### `POP`

- **Description**: Removes the top value from the stack.
- **Stack Behavior**: Pops 1 value, pushes 0 values.

## Constants Loading

### `ICONST_0`, `ICONST_1`, `FCONST_0`, `FCONST_1`

- **Description**: Pushes specified constant values onto the stack (`0` or `1`, integer or float accordingly).
- **Stack Behavior**: Pops 0 values, pushes 1 value.

### `LOADCONST`

- **Description**: Loads a specific constant from the constants pool and pushes it onto the stack.
- **Operands**:
  The index of the constant in the constant pool.
- **Stack Behavior**: Pops 0 values, pushes 1 value.

## Type Conversion

### `I2F`

- **Description**: Converts the top integer on the stack to a float.
- **Stack Behavior**: Pops 1 value, pushes the result (1 value).

### `F2I`

- **Description**: Converts the top float on the stack to an integer.
- **Stack Behavior**: Pops 1 value, pushes the result (1 value).

## Branching

### `GOTO`, `IFEQ`, `IFNEQ`, `IFGT`, `IFGE`, `IFLT`, `IFLE`

- **Description**: Alters the flow of execution based on conditionals or unconditionally (`GOTO`).
- **Stack Behavior**:
  - `GOTO`: Pops 0 values, pushes 0 values.
    - **Operands**: The jump address, 2 bytes long.
  - Conditionals: Pops 2 values for comparison, pushes 0 values.

## Function Call

### `CALL`

- **Description**: Calls a function by module and function indices.
- **Operands**:
  - The module index, 2 bytes long.
  - The function index, 2 bytes long.
- **Stack Behavior**: Depends on the function's definition. Generally, it pops arguments and pushes the result if any.

## Local Variables

### `LOAD`, `STORE`

- **Stack Behavior**:
  - `LOAD`:
    - **Operands**: The local variable index.
  - `STORE`:
    - **Operands**: The local variable index.

## Memory Operations

### `NEW_OBJECT`, `NEW_ARRAY`, `ARRAY_GET`, `ARRAY_SET`

- **Description**: Perform various memory operations like loading and storing values, creating objects and arrays, and accessing array elements.
  - `NEW_OBJECT`, `NEW_ARRAY`: Pops 0 or 1 value (size for arrays), pushes 1 value (reference to created object/array).
  - `ARRAY_GET`: Pops 2 values (array reference and index), pushes 1 value (element at index).
  - `ARRAY_SET`: Pops 3 values (array reference, index, and the value to set), pushes 0 values.

## Field Operations

### `SET_FIELD`, `GET_FIELD`

- **Description**: Sets or gets the value of a field in an object.
- **Stack Behavior**:
  - `SET_FIELD`: Pops 3 values (object reference, field identifier, and value to set), pushes 0 values.
  - `GET_FIELD`: Pops 2 values (object reference and field identifier), pushes 1 value (field value).

## Return Operations

### `RET`, `RETURN`

- **Description**: Returns control flow from a function, optionally with a value.
- **Stack Behavior**:
  - `RET`: Pops 0 values, pushes 0 values.
  - `RETURN`: Pops 1 value (the return value), pushes 0 values.

## Misc

### `PUSH_BYTE` and `PUSH_SHORT`

- **Description**: Pushes a literal byte or short value onto the stack.
- **Operands**:
  - **PUSH_BYTE**:
    - **Operands**: A single byte value.
    - **Usage**: The next byte specifies the Integer value to push onto the stack.
  - **PUSH_SHORT**:
    - **Operands**: Two bytes forming a short value.
    - **Usage**: Uses next two bytes to form a Integer value to push onto the stack.
