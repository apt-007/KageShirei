import {
    createTheme,
    MantineProvider,
} from "@mantine/core";
import "./tailwind.css";

import type { Metadata } from "next";
import { Nunito } from "next/font/google";
import "@mantine/core/styles.css";
import { ReactNode } from "react";

const nunito = Nunito({
    subsets: [
        "cyrillic-ext",
        "latin-ext",
        "vietnamese",
    ],
    variable: "--font-app",
});

const theme = createTheme({
    fontFamily: nunito.style.fontFamily,
    primaryColor: "violet",
});

export const metadata: Metadata = {
    title: "Create Next App",
    description: "Generated by create next app",
};

interface LayoutProps {
    children: ReactNode;
}

export default function RootLayout(
    {
        children,
    }: LayoutProps,
) {
    return (
        <html lang="en"
              className="dark overflow-hidden"
              suppressHydrationWarning
        >
        <body className={ nunito.className }>
            <MantineProvider theme={ theme }
                             defaultColorScheme={ "dark" }
                             forceColorScheme={ "dark" }
                             withCssVariables
            >
                { children }
            </MantineProvider>
        </body>
        </html>
    );
}
