import styles from './HomePage.module.scss';

import { Button, Navbar, NavbarGroup, NavbarHeading } from '@blueprintjs/core';
import { useDarkThemeContext } from '../../../contexts/';
import { CumulativeScoreGraph } from '../../controls/CumulativeScoreGraph/CumulativeScoreGraph';

export const HomePage = () => {

    const { useDarkTheme, setUseDarkTheme } = useDarkThemeContext();

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
                <CumulativeScoreGraph />
            </div>
        </div>
    );
}