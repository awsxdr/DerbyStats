import { createContext, useContext, useState, PropsWithChildren, useEffect } from 'react';
import { getCookie, setCookie } from 'typescript-cookie';
import styles from './DarkThemeContext.module.scss';

export interface DarkThemeContextProps {
    useDarkTheme: boolean;
    setUseDarkTheme: (useDarkTheme: boolean) => void;
};

const DefaultDarkThemeContext = (): DarkThemeContextProps => ({
    useDarkTheme: false,
    setUseDarkTheme: _ => {},
});

const DarkThemeContext = createContext<DarkThemeContextProps>(DefaultDarkThemeContext());

export const useDarkThemeContext = () => useContext(DarkThemeContext);

export const DarkThemeContextProvider = ({ children }: PropsWithChildren) => {

    const [useDarkTheme, setUseDarkTheme] = useState(false);

    const handleDarkThemeSet = (value: boolean) => {
        setUseDarkTheme(value);
        setCookie('darkmode', value);
    };

    useEffect(() => {
        setUseDarkTheme(getCookie('darkmode')?.toLowerCase() === 'true');
    }, [setUseDarkTheme])

    return (
        <>
            <div className={`${useDarkTheme && 'bp5-dark'}`}>
                <DarkThemeContext.Provider value={{ useDarkTheme, setUseDarkTheme: handleDarkThemeSet }}>
                    { children }
                </DarkThemeContext.Provider>
            </div>
            { useDarkTheme && <div className={styles.darkThemeBackground}></div> }
        </>
    ); }