import { useEffect, useState } from "react";
import { useDarkThemeContext } from "../contexts"

type ColorOptions = {
    light: string,
    dark: string,
};

const COLORS = [
    ["black",       "#000", "#666" ],
    ["white",       "#ddd", "#fff" ],
    ["grey",        "#888", "#888" ],
    ["gray",        "#888", "#888" ],
    ["red",         "#f00", "#f33" ],
    ["pink",        "#faa", "#faa" ],
    ["orange",      "#f80", "#fa0" ],
    ["yellow",      "#dd0", "#ff0" ],
    ["brown",       "#850", "#a60" ],
    ["gold",        "#db0", "#fc0" ],
    ["green",       "#0b0", "#0f0" ],
    ["teal",        "#0bb", "#0ff" ],
    ["turquoise",   "#0bb", "#0ff" ],
    ["blue",        "#00d", "#66f" ],
    ["purple",      "#80f", "#a3f" ],
    ["lilac",       "#86f", "#c8f" ],
]
.reduce((a, o) => 
    (a[o[0]] = { light: o[1], dark: o[2] }, a),
    {} as Record<string, ColorOptions>);

export const useTeamColor = (colorName: string): string => {
    const { useDarkTheme } = useDarkThemeContext();
    const [ color, setColor ] = useState("#888");
    colorName = colorName.toLocaleLowerCase();

    useEffect(() => {
        setColor(useDarkTheme ? COLORS[colorName]?.dark : COLORS[colorName]?.light ?? "#888");
    }, [ useDarkTheme, setColor ])
    
    return color;
};