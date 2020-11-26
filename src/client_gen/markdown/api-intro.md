## API description

In language pack for your programming language you can find file named `MyStrategy.<ext>`/`my_strategy.<ext>`.
This file contains class `MyStrategy` with `get_action` method, where your strategy's logic should be implemented.

This method will be called each tick.

The method takes following arguments:

- Player view — all the information you have about current game's state,
- Debug interface — this object allows you to do send debug commands to the app and receive debug state from inside your strategy code. Note that this is unavailable when testing your strategy on the server, or using the app in batch mode. This is for local debugging only.

The method should return the action you desire to perform this tick.

For debugging purposes, there is also another method — `debug_update`, that has same parameters, and is called continiously while the app is running (not in batch mode), if the client is waiting for the next tick. There will always be at least one debug update between ticks.

## Objects description

In this section, some fields may be absent (denoted as `Option<type>`).
The way this is implemented depends on the language used.
If possible, a dedicated optional (nullable) type would be used,
otherwise other methods may be used (like a nullable pointer type).

Some objects may take one of several forms. The way it is implemented depends on the language.
If possible, a dedicated sum (algebraic) data type is used,
otherwise other methods may be used (like variants being classes inherited from abstract base class).
