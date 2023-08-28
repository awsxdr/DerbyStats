import styles from './HomePage.module.scss';

import { Label, LineChart, Line, XAxis, YAxis } from 'recharts';

import { Button, Navbar, NavbarGroup, NavbarHeading } from '@blueprintjs/core';
import { useDarkThemeContext } from '../../../contexts/';

export const HomePage = () => {

    const { useDarkTheme, setUseDarkTheme } = useDarkThemeContext();

    const data = [
        { jam: 1, team1Score: 4, team2Score: 8 },
        { jam: 2, team1Score: 4, team2Score: 8 },
        { jam: 3, team1Score: 4, team2Score: 12 },
        { jam: 4, team1Score: 8, team2Score: 15 },
        { jam: 5, team1Score: 10, team2Score: 16 },
        { jam: 6, team1Score: 12, team2Score: 20 },
        { jam: 7, team1Score: 12, team2Score: 22 },
        { jam: 8, team1Score: 20, team2Score: 22 },
        { jam: 9, team1Score: 20, team2Score: 22 },
        { jam: 10, team1Score: 24, team2Score: 26 },
        { jam: 11, team1Score: 24, team2Score: 29 },
        { jam: 12, team1Score: 24, team2Score: 33 },
        { jam: 13, team1Score: 29, team2Score: 41 },
        { jam: 14, team1Score: 31, team2Score: 43 },
        { jam: 15, team1Score: 34, team2Score: 43 },
        { jam: 16, team1Score: 42, team2Score: 54 },
        { jam: 17, team1Score: 42, team2Score: 54 },
        { jam: 18, team1Score: 42, team2Score: 58 },
        { jam: 19, team1Score: 42, team2Score: 62 },
        { jam: 20, team1Score: 45, team2Score: 70 },
        { jam: 21, team1Score: 45, team2Score: 74 },
        { jam: 22, team1Score: 45, team2Score: 77 },
        { jam: 23, team1Score: 49, team2Score: 80 },
        { jam: 24, team1Score: 50, team2Score: 88 },
        { jam: 25, team1Score: 62, team2Score: 88 },
        { jam: 26, team1Score: 62, team2Score: 88 },
    ];

    return (
        <div className={`${useDarkTheme && 'bp5-dark'}`}>
            <Navbar fixedToTop>
                <NavbarGroup>
                    <NavbarHeading>🛼 Derby Stats</NavbarHeading>
                </NavbarGroup>
                <NavbarGroup align='right'>
                    <Button className="bp5-minimal" icon={useDarkTheme ? 'flash' : 'moon'} onClick={() => setUseDarkTheme(!useDarkTheme)} />
                </NavbarGroup>
            </Navbar>
            <div className={styles.homeContent}>
                <LineChart data={data} width={800} height={600}>
                    <Line type="monotone" stroke='#ff4400' strokeWidth={3} dataKey="team1Score" />
                    <Line type="monotone" stroke='#00bb22' strokeWidth={3} dataKey="team2Score" />
                    <XAxis dataKey="jam">
                        <Label value="Jam #" />
                    </XAxis>
                    <YAxis>
                        <Label value="Cumulative score" angle={-90} />
                    </YAxis>
                </LineChart>
            </div>
        </div>
    );
}