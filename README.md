## 📖 Syntax Documentation

| Feature | MommyLang Syntax | C Equivalent | Description |
| :--- | :--- | :--- | :--- |
| **Declare Variable** | `mayihave [VAL] in [NAME] as [TYPE]` | `int name = val;` | Politely requests memory. |
| **Addition** | `add [NAME] with [VAL]` | `name = name + val;` | Adds a value to a variable. |
| **Print Text** | `say "[MESSAGE]"` | `printf("%s\n", "msg");` | Prints a string. **Must use quotes.** |
| **Print Variable** | `say [NAME]` | `printf("%d\n", name);` | Prints a number/variable. **No quotes.** |
| **If Condition** | `ask if [CONDITION]` | `if (cond) {` | Starts a logic check. |
| **Else** | `or` | `} else {` | The alternative path. |
| **Loop** | `punishme [AMOUNT]` | `for(int i=0; i<amt; i++){` | Repeats the block `amount` times. |
| **End Block** | `done` | `}` | Closes an `ask`, `or`, or `punishme`. |
| **Exit** | `leave` | `return 0;` | Ends the program. |
