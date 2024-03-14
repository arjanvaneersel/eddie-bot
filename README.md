# Eddie the bot

Copyright (c) 2024, Arjan van Eersel.

Eddie is a Telegram and Discord bot that interacts with Substrate nodes.

## License

```
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
```

## State

Under heavy development, don't use in production.

## Features

- [x] Discord support
- [x] Telegram support
- [ ] Multinet faucets
- [ ] Tipping

## Contributing

Proper documentation will be created eventually.

### Adding commands

#### 1. Add the command at bot level (`eddie-lib`)

- 1. Add the command's functionality in `functions.rs`
- 2. Add an option to the Call enum in `call.rs`
- 3. Add the dispatch logic to the Dispatch implementation in `call.rs`

#### 2. Add the command for Discord (`transport/discord`)

- 1. Add the command's logic for Discord to the handler in `commands.rs`. This part should dispatch the call to the bot. At the moment only poise `prefix_command` and `slash_command` are used. Admin commands should NOT use `slash_command`.
- 2. Add the command to the list of options around like 52 in `discord.rs`. Please keep in mind that it sometimes takes a while before slash commands are registered in Discord.

#### 3. Add the command for Telegram (`transport/telegram`)

- 1. Add the command to the Command enum in `telegram.rs`
- 2. Add the command's logic in the `process` function of `TelegramTransport`. This part should dispatch the call to the bot.
