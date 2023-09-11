import styles from './HomePage.module.scss';

import { Button, Classes, Drawer, DrawerSize, Navbar, NavbarHeading, Tab, TabId, Tabs } from '@blueprintjs/core';
import { useDarkThemeContext } from '../../../contexts/';
import { CumulativeScoreTab } from '../tabs';
import { useState } from 'react';
import { CumulativeScoreGraph, TeamPenaltiesChart } from '../../controls';

export const HomePage = () => {

    const { useDarkTheme, setUseDarkTheme } = useDarkThemeContext();
    const [ selectedTabId, setSelectedTabId ] = useState("cumulativeScore");
    const [ isMenuOpen, setIsMenuOpen ] = useState(false);

    const handleTabChange = (tabId: TabId) => {
        setSelectedTabId(tabId as string);
        setIsMenuOpen(false);
    };

    return (
        <div className={`${useDarkTheme && 'bp5-dark'}`}>
            <Navbar fixedToTop>
                <Navbar.Group>
                    <Button icon='menu' minimal onClick={() => setIsMenuOpen(true)} />
                </Navbar.Group>
                <Navbar.Group>
                    <NavbarHeading>🛼 Derby Stats</NavbarHeading>
                </Navbar.Group>
                <Navbar.Group align='right'>
                    <Button className="bp5-minimal" icon={useDarkTheme ? 'flash' : 'moon'} onClick={() => setUseDarkTheme(!useDarkTheme)} />
                </Navbar.Group>
            </Navbar>
            <div className={styles.bodyContainer}>
                {
                    (selectedTabId === "cumulativeScore") ? <CumulativeScoreGraph />
                    : (selectedTabId === "teamPenaltiesByType") ? <TeamPenaltiesChart />
                    : <>Unrecognized graph</>
                }
            </div>
            <Drawer isOpen={isMenuOpen} onClose={() => setIsMenuOpen(false)} position='left' hasBackdrop={false} size={DrawerSize.SMALL}>
                <div className={Classes.DRAWER_BODY} >
                    <Tabs className={styles.drawerTabs} renderActiveTabPanelOnly animate vertical onChange={handleTabChange} selectedTabId={selectedTabId} fill>
                        <h5>Scores</h5>
                        <Tab id="cumulativeScore" title="Cumulative Score" />
                        <h5>Penalties</h5>
                        <Tab id="teamPenaltiesByType" title="Team penalties by type" />
                    </Tabs>
                </div>
            </Drawer>
        </div>
    );
}