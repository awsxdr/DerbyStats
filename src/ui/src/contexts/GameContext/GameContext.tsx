import { createContext, useContext, useMemo, useState, PropsWithChildren, useEffect } from 'react';
import { useGlobalStateSocket } from '../../hooks';
import { SetURLSearchParams, useSearchParams } from 'react-router-dom';

type Team = {
    name: string,
    color: string,
}

type Game = {
    id: string,
    isCurrent: boolean,
    homeTeam: Team,
    awayTeam: Team,
    startTime: number,
}

type GamesUpdate = {
    body: Game[];
}

export interface GameContextProps {
    games: Game[];
    selectedGame?: Game;

    setSelectedGame: (game?: Game) => void;
};

const DefaultGameContext = (): GameContextProps => ({
    games: [],
    setSelectedGame: () => {},
});

const GameContext = createContext<GameContextProps>(DefaultGameContext());

export const useGameContext = () => useContext(GameContext);

export const GameContextProvider = ({ children }: PropsWithChildren) => {

    const [games, setGames] = useState<Game[]>([]);
    const [selectedGame, setSelectedGame] = useState<Game>();

    const [setSearchParams, setSetSearchParams] = useState<SetURLSearchParams>();
    const [searchParams, setRouteSearchParams] = useState<URLSearchParams>();

    useEffect(() => {
        const [searchParams, setSearchParams] = useSearchParams();
        setSetSearchParams(setSearchParams);

        const gameId = searchParams.get('gameId');

        useGlobalStateSocket<GamesUpdate>("Games", update => {
            setGames(update.body.sort((a, b) => a.isCurrent ? -1 : b.startTime - a.startTime));
    
            const game = update.body.find(({ id }) => gameId === id);
    
            if(game) {
                setSelectedGame(game);
            } else {
                setSearchParams && searchParams && setSearchParams({ ...Object.fromEntries(searchParams.entries()), gameId: update.body.find(({ isCurrent }) => isCurrent)!.id });
            }
        }, [setGames, setSelectedGame]);
    }, [setSetSearchParams, setRouteSearchParams]);

    const contextSetSelectedGame = useMemo(() => (game?: Game) => {
        setSelectedGame(game);
        setSearchParams && searchParams && setSearchParams({ ...Object.fromEntries(searchParams.entries()), gameId: game?.id ?? '' });
    }, [setSelectedGame, setSearchParams]);

    return (
        <GameContext.Provider value={{games, selectedGame, setSelectedGame: contextSetSelectedGame}}>
            { children }
        </GameContext.Provider>
    )
}