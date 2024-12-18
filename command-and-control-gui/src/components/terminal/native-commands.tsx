import Ansi from "ansi-to-react";
import {
    Dispatch,
    JSX,
    SetStateAction,
} from "react";

interface CommandHandlerArguments {
    args: string[];
    cwd: string;
    username: string;
    hostname: string;
    set_cwd: (cwd: string) => void;
    set_terminal_fragments: Dispatch<SetStateAction<JSX.Element[]>>;
    terminal_fragments: JSX.Element[];
    hooks: {
        dropTerminalHandle: (hostname: string) => void;
        addTerminalHandle: (hostname: string, cwd: string, id: string) => void;
    };
}

type CommandHandler = (args: CommandHandlerArguments) => void | Promise<void>;

interface CommandDefinition {
    description: string;
    handler: CommandHandler;
}

export type NativeHandler = "clear" | "exit" | "open_session";

export const TERMINAL_EMULATOR_INTERNAL_HANDLES = [
    "__TERMINAL_EMULATOR_INTERNAL_HANDLE_CLEAR__",
    "__TERMINAL_EMULATOR_INTERNAL_HANDLE_EXIT__",
    "__TERMINAL_EMULATOR_INTERNAL_HANDLE_OPEN_SESSIONS__",
] as const;

export const INTERNAL_HANDLES_LOOKUP: { [x in typeof TERMINAL_EMULATOR_INTERNAL_HANDLES[number]]: NativeHandler } = {
    __TERMINAL_EMULATOR_INTERNAL_HANDLE_CLEAR__:         "clear",
    __TERMINAL_EMULATOR_INTERNAL_HANDLE_EXIT__:          "exit",
    __TERMINAL_EMULATOR_INTERNAL_HANDLE_OPEN_SESSIONS__: "open_session",
};

export const NATIVE_COMMANDS: { [x in NativeHandler]: CommandDefinition } = {
    clear:        {
        description: "Clear the terminal",
        handler: ({ set_terminal_fragments }) => {
            set_terminal_fragments([]);
        },
    },
    exit:         {
        description: "Exit the current terminal",
        handler:     ({
            hooks,
            hostname,
            set_terminal_fragments,
        }) => {
            // Using the hostname as the terminal handle avoids the possibility to exit the global terminal as it does
            // not follow the common hostname convention. This is a security measure to prevent the user from closing
            // the global terminal by accident. The global terminal is the terminal that is always open and is used to
            // run global commands. Even if the global terminal cannot be closed dropping it will break the line input,
            // this is the reason why the global terminal is not closed.
            if (hostname === "KageShirei") {
                // if the terminal is the global terminal, do not exit and show an error message
                set_terminal_fragments(old => [
                    ...old,
                    <pre key={ `${ hostname }-out-${ old.length + 1 }` }
                         className="break-all"
                    >
                        <Ansi>
                            &#x1B;[1m&#x1B;[31mError:&#x1B;[0m The global terminal cannot be closed.
                        </Ansi>
                    </pre>,
                ]);

                return;
            }

            hooks.dropTerminalHandle(hostname);
        },
    },
    open_session: {
        description: "Open a new terminal session",
        handler: ({
            hooks,
            cwd,
            hostname,
            args,
            set_terminal_fragments,
        }) => {
            console.log(args);

            let ansi_string = `[\x1b[32;1m OK \x1b[0m] Opening new session for ${ hostname }`;
            set_terminal_fragments(old => [
                ...old,
                <div key={ `${ hostname }-out-${ old.length + 1 }` }
                     className="break-all whitespace-pre-wrap"
                >
                    <Ansi>
                        { ansi_string }
                    </Ansi>
                </div>,
            ]);

            hooks.addTerminalHandle(hostname, cwd, args[0]);
        },
    },
};