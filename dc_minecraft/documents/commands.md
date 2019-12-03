# Minecraft commands

## scoreboard objectives add

```minecraft
/scoreboard objectives add <objective> <criteria> [<displayName>]
```

This command adds an objective that tracks a specified criteria.
It can be given an optional display name. Both messages and scoreboard displays use this name.
When refering to the objective in a command, use the actual objective name.
Multiple objectives can have the same display name, but objective names must be unique.
Attempting to add an objective when one with that name already exists will result in an error.