import styles from './HomePage.module.scss';

import { Button, Classes, Drawer, DrawerSize, Navbar, NavbarHeading, TabId } from '@blueprintjs/core';
import { useDarkThemeContext } from '../../../contexts/';
import { useState } from 'react';
import { ContentsList, GamesList, GraphTabIds, GraphTabs } from '../../controls';
import { useSearchParams } from 'react-router-dom';

export const HomePage = () => {

    const [searchParams, setSearchParams] = useSearchParams();
    const { useDarkTheme, setUseDarkTheme } = useDarkThemeContext();
    const [ isMenuOpen, setIsMenuOpen ] = useState(false);
    const selectedTabId = parseInt(searchParams.get("graph") || "0") as GraphTabIds;

    const handleTabChange = (tabId: TabId) => {
        setIsMenuOpen(false);
        setSearchParams({ ...Object.fromEntries(searchParams.entries()), graph: tabId.toString() });
    };

    return (
        <div className={`${useDarkTheme && 'bp5-dark'}`}>
            <Navbar fixedToTop>
                <Navbar.Group>
                    <Button icon='menu' minimal onClick={() => setIsMenuOpen(true)} />
                </Navbar.Group>
                <Navbar.Group>
                    <NavbarHeading>🛼 Derby Stats - <span className="bp5-text-overflow-ellipsis">{GraphTabs.get(selectedTabId)?.title || "Not found"}</span></NavbarHeading>
                </Navbar.Group>
                <Navbar.Group align='right'>
                    <Button className="bp5-minimal" icon={useDarkTheme ? 'flash' : 'moon'} onClick={() => setUseDarkTheme(!useDarkTheme)} />
                </Navbar.Group>
            </Navbar>
            <GamesList />
            <div className={styles.bodyContainer}>
                {
                    GraphTabs.get(selectedTabId)?.component() || <>Unrecognized graph</>
                }
            </div>
            <Drawer isOpen={isMenuOpen} onClose={() => setIsMenuOpen(false)} position='left' hasBackdrop={false} size={DrawerSize.SMALL}>
                <div className={Classes.DRAWER_BODY} >
                    <ContentsList className={styles.drawerTabs} onTabSelected={handleTabChange} selectedTabId={selectedTabId} />
                </div>
            </Drawer>
        </div>
    );
}