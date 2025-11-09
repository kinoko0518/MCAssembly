# 👋 Welcome to MCAssembly!
This is a very small assembly language for operating scoreboards in Minecraft.

## 🪛 How to use
There are two ways to provide the source code:
### Compiling an Existing File
You can pass a file path as an argument. The command will read, compile, and print the result.
```
mcasm <path_to_your_file>
```
### Writing Code Interactively
```
mcasm
```
If you run mcasm without providing a path, it will:
  
- Automatically create a temporary file.
- Open this temporary file using your system's default terminal text editor (such as vim or nano).
- Wait for you to write your code.

Once you save and close the editor, it will compile the content of that temporary file and print the result to standard output.

## 📦 Supported Types
| Name | Size | Remarks |
| --- | --- | --- |
| `byte` | 1byte | |
| `short` | 2byte | |
| `int` | 4byte | Literal's default. |
| `long` | 8byte | Able to assign to scoreboards, but it may occurs a overflow. |
| `float` | 4byte | Can't assign to scoreboards. |
| `double` | 8byte  | Can't assign to scoreboards. |

## ✏️ Syntax
### Comment
You can comment out a line adding `//` to the first of line.

### Scoreboard
```
objective::selector
```
To reference a scoreboard, it must follow the format above.

### Storage
#### NBTStorage Name
```
namespace:name
```
It's easy!

#### NBT Path
```
path.to.data[0]::<int>
```
You can use the same syntax as in Minecraft for the part before the `::`.

You can optionally add a type specification, starting with `::` and a type name in `<>`.

It's optional. If you don't add a type specification, it will be treated as `int`.

## 🏗️ Mnemonics
| Mnemonic | Operands | Remarks |
| --- | --- | --- |
| MOV | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Copy the right operand's value to the left operand |
| ADD | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Add the right operand's value to the left operand |
| SUB | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Subtract the right operand's value to the left operand |
| MUL | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Multiply the right operand's value to the left operand |
| DIV | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Divide the right operand's value to the left operand |
| SUR | `target: Scoreboard` `source: (Numeric \| Scoreboard)` | Modulo  the right operand's value to the left operand |
| NTS | `target: Scoreboard` `source: Storage` `reference_path: NBTPath` `magnification: Numeric` | Reads a value from the reference_path within the source storage. This value is then multiplied by magnification and stored in the target scoreboard. |
| STN | `target: Storage` `target_path: NBTPath` `source: Scoreboard` `magnification: Numeric` | Copies the value from the source scoreboard, multiplies it by magnification, and stores the result in the target_path of the target storage. |
| REL | `target: Scoreboard` | Resets the score for the player specified in the target scoreboard. This removes that player's entry from the objective. (Equivalent to /scoreboard players reset) |

## 📄 Sample Codes
### 1. Calcation between Scores
**Setup:**
```
scoreboard objective add mcasm dummy
```
**MCAssembly:**
```
// B, C = 5, 6
MOV mcasm::#B 5
MOV mcasm::#C 6

// A = (B + C) * 3
MOV mcasm::#A mcasm::#B
ADD mcasm::#A mcasm::#C
MUL mcasm::#A 3
```
**Result:**
```
scoreboard players set #B mcasm 5
scoreboard players set #C mcasm 6
scoreboard players operation #A mcasm = #B mcasm
scoreboard players operation #A mcasm += #C mcasm
scoreboard players set LITERAL_SCORE_CONVERSION MC_ASM 3
scoreboard players operation #A mcasm *= LITERAL_SCORE_CONVERSION MC_ASM
```
### 2. Multiply Storage
**Setup:**
```
scoreboard objective add mcasm dummy
data merge storage mcasm:foo {foo: {bar: {buz: 3.14}}}
```
**MCAssembly:**
```
// Move the value to a
// temp scoreboard for calcation
NTS mcasm::#temp mcasm:foo foo.bar.buz::<float> 1000
// Make value 2x
MUL mcasm::#temp 2
// Store result
STN mcasm:foo foo.bar.buz::<float> mcasm::#temp 0.001
```
**Result:**
```
execute store result score #temp mcasm run data get mcasm:foo foo.bar.buz 1000
scoreboard players set LITERAL_SCORE_CONVERSION MC_ASM 2
scoreboard players operation #temp mcasm *= LITERAL_SCORE_CONVERSION MC_ASM
execute store result storage mcasm:foo foo.bar.buz float 0.001 run scoreboard players get #temp mcasm
```
