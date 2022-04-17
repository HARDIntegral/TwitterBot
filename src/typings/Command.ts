import { ChatInputApplicationCommandData, CommandInteractionOptionResolver } from "discord.js"
import { ExtendedClient } from "../structures/client";

interface RunOptions {
    client: ExtendedClient,
    interaction: CommandInteraction,
    args: CommandInteractionOptionResolver
}

type RunFunction = (options: RunOptions) => any;

export type CommandType = {
    userPermission?: PermissionResolvable[];
} & ChatInputApplicationCommandData
