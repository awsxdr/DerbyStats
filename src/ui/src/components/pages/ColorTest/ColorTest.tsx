import { Button, Navbar } from "@blueprintjs/core";
import { useDarkThemeContext } from "../../../contexts";
import { useTeamColor } from "../../../hooks/team-color"

const colors = [
    "Black",
    "White",
    "Grey",
    "Gray",
    "Red",
    "Pink",
    "Orange",
    "Yellow",
    "Brown",
    "Gold",
    "Green",
    "Teal",
    "Turquoise",
    "Blue",
    "Purple",
    "Lilac",
]

interface IColorRowProps {
    colorName: string,
};

const ColorRow = ({ colorName }: IColorRowProps) => {
    const color = useTeamColor(colorName);
    const { useDarkTheme, setUseDarkTheme } = useDarkThemeContext();

    return (
        <div className={`${useDarkTheme && 'bp5-dark'}`}>
            <Navbar fixedToTop>
                <Navbar.Group align='right'>
                    <Button className="bp5-minimal" icon={useDarkTheme ? 'flash' : 'moon'} onClick={() => setUseDarkTheme(!useDarkTheme)} />
                </Navbar.Group>
            </Navbar>
            <div style={{ display: 'inline-flex', alignItems: 'center' }}>
                <span style={{ width: '10vw', display: 'inline-block', color: color }}>{colorName}</span>
                <span style={{ width: '20vw', height: '30px', backgroundColor: color, display: 'inline-block'}}>&nbsp;</span>
            </div>
        </div>
    )
}

export const ColorTest = () => {

    return (
        <div>
            {colors.map(colorName => <ColorRow colorName={colorName} />)}
        </div>
    )
}